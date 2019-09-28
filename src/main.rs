extern crate web_rust;

use std::fs::File;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::thread;
use env_logger;
use log::info;
use web_rust::response::{self, HttpResponse};
use web_rust::request;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("server begin");

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
               thread::spawn(move || {
                   handle_client(stream)
               });
            }
            Err(_) => { panic!("connection failed")}
        }
    }
}

fn handle_client(stream: TcpStream) {
    info!("handle_client begin");
    let (result, mut stream) = request::from_stream(stream);
    let res = match result {
        Err(_) => response::internal_server_error(),
        Ok(req) => match &*req.method {
            "GET" => get_operation(&req.path),
            "HEAD" => head_operation(&req.path),
            _ => response::not_implemented()
        }
    };
    res.write_stream(&mut stream);
}

fn get_operation(path: &String) -> HttpResponse {
    info!("get_operation begin");

    let path_fragments: Vec<&str> = path.split('/').collect();
    let path_type = path_fragments[1];
    if path_fragments.len() < 3 {
        response::not_found()
    } else {
        let path: String = path_fragments.into_iter().skip(2).collect();
        match path_type {
            "html" => get_operation_html(&path),
            "ssr" => get_operation_ssr(&path),
            _ => response::not_found(),
        }
    }
}
fn get_operation_html(path: &String) -> HttpResponse {
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
fn get_operation_ssr(path: &String) -> HttpResponse {
    info!("get_operation_ssr begin");

    let body = format!("<p>this is SSR</p><p>path: {}</p>", path);
    response::ok(body, true)
}

// TODO: head_operation and get_operation are similar.
fn head_operation(path: &String) -> HttpResponse {
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




















