use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use log::info;

use crate::request::HttpRequest;
use crate::response::HttpResponse;
use crate::web_server::{WebServerStatus, ssr};

pub struct Dispatcher {

}

impl Dispatcher {
    pub fn dispatch(&self, req: &HttpRequest, status: &mut WebServerStatus) -> HttpResponse {
        let ll = &req.lead_line;
        match &*ll.method {
            "GET" => self.get_operation(&ll.path, req, status),
            "POST" => self.post_operation(&ll.path, req, status),
            "HEAD" => self.head_operation(&ll.path),
            _ => HttpResponse::not_implemented(),
        }
    }

    fn get_operation(&self, path: &String, req: &HttpRequest, status: &mut WebServerStatus) -> HttpResponse {
        info!("get_operation begin");

        let path_fragments: Vec<&str> = path.split('/').collect();
        let path_type = path_fragments[1];
        if path_fragments.len() < 3 {
            HttpResponse::not_found()
        } else {
            let path: String = path_fragments.into_iter().skip(2).collect();
            match path_type {
                "html" => self.get_operation_html(&path),
                "ssr" => ssr::dispatch_ssr(&path, req, status),
                _ => HttpResponse::not_found(),
            }
        }
    }

    pub fn get_operation_html(&self, path: &String) -> HttpResponse {
        info!("get_operation_html begin");

        let path = PathBuf::from(format!("/Users/matsuzakirei/src/github.com/ReiMatsuzaki/web-rust/www{}", path));
        let open_file = File::open(&path);
        match open_file {
            Ok(file) => {
                let mut body = String::new();
                let mut file = file;
                match file.read_to_string(&mut body) {
                    Ok(_) => HttpResponse::ok(body, true),
                    Err(_) => HttpResponse::internal_server_error(),
                }
            },
            Err(_) => {
                HttpResponse::not_found()
            },
        }
    }

    fn post_operation(&self, path: &String, req: &HttpRequest, status: &mut WebServerStatus) -> HttpResponse {
        info!("post_operation begin");

        let path_fragments: Vec<&str> = path.split('/').collect();
        let path_type = path_fragments[1];
        if path_fragments.len() < 3 {
            HttpResponse::not_found()
        } else {
            let path: String = path_fragments.into_iter().skip(2).collect();
            match path_type {
                "html" => self.get_operation_html(&path),
                "ssr" => ssr::dispatch_ssr(&path, &req, status),
                _ => HttpResponse::not_found(),
            }
        }
    }

    // TODO: head_operation and get_operation are similar.
    pub fn head_operation(&self, path: &String) -> HttpResponse {
        info!("head_operation_ssr begin");

        let path = PathBuf::from(format!("/Users/matsuzakirei/src/github.com/ReiMatsuzaki/web-rust/www{}", path));
        let open_file = File::open(&path);
        match open_file {
            Ok(file) => {
                let mut body = String::new();
                let mut file = file;
                match file.read_to_string(&mut body) {
                    Ok(_) => HttpResponse::ok(body, false),
                    Err(_) => HttpResponse::internal_server_error(),
                }
            },
            Err(_) => {
                HttpResponse::not_found()
            },
        }
    }

}