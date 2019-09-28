use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use log::info;

use crate::response;
use crate::response::HttpResponse;
use crate::request::HttpRequest;
use crate::ssr;
use crate::request;

pub struct Dispatcher {

}

impl Dispatcher {
    pub fn dispatch(&self, req: HttpRequest) -> HttpResponse {
        let ll = req.lead_line;
        match &*ll.method {
            "GET" => self.get_operation(&ll.path, req.body),
            "HEAD" => self.head_operation(&ll.path),
            _ => response::not_implemented(),
        }
    }

    fn get_operation(&self, path: &String, req_body: request::Body) -> HttpResponse {
        info!("get_operation begin");

        let path_fragments: Vec<&str> = path.split('/').collect();
        let path_type = path_fragments[1];
        if path_fragments.len() < 3 {
            response::not_found()
        } else {
            let path: String = path_fragments.into_iter().skip(2).collect();
            match path_type {
                "html" => self.get_operation_html(&path),
                "ssr" => ssr::dispatch_ssr(&path, &req_body),
                _ => response::not_found(),
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
                    Ok(_) => response::ok(body, true),
                    Err(_) => response::internal_server_error(),
                }
            },
            Err(_) => {
                response::not_found()
            },
        }
    }

    pub fn get_operation_ssr(&self, path: &String) -> HttpResponse {
        info!("get_operation_ssr begin");

        let body = format!("<p>this is SSR</p><p>path: {}</p>", path);
        response::ok(body, true)
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
                    Ok(_) => response::ok(body, false),
                    Err(_) => response::internal_server_error(),
                }
            },
            Err(_) => {
                response::not_found()
            },
        }
    }

}