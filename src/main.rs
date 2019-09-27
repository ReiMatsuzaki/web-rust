extern crate web_rust;

use std::fs::File;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::thread;
use web_rust::response::{self, HttpResponse};
use web_rust::request;

fn main() {
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

fn head_operation(path: &String) -> HttpResponse {
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




















