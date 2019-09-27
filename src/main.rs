extern crate web_rust;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::thread;
use web_rust::response;
use web_rust::response::HttpResponse;


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
    println!("Hello, world!");
}

fn handle_client(tcp_stream: TcpStream) {
    let mut stream: BufReader<TcpStream> = BufReader::new(tcp_stream);

    let mut first_line = String::new();
    if let Err(err) = stream.read_line(&mut first_line) {
        panic!("error during receive a line: {}", err);
    }

    let mut params = first_line.split_whitespace();
    let method = params.next();
    let path = params.next();
    let res = match (method, path) {
        (Some("GET"), Some(path)) => {
            get_operation(path)
        }
        _ => response::not_implemented()
    };

    let mut tcp_stream = stream.into_inner();
    res.write_stream(&mut tcp_stream);
}

fn get_operation(path: &str) -> HttpResponse {
    let path = PathBuf::from(format!("/Users/matsuzakirei/src/github.com/ReiMatsuzaki/web-rust/www{}", path));
    let open_file = File::open(&path);
    match open_file {
        Ok(file) => {
            let mut body = String::new();
            let mut file2 = file; // TODO: this code seems illegal
            match file2.read_to_string(&mut body) {
                Ok(_) => response::ok(body),
                Err(_) => response::internal_server_error(),
            }
        },
        Err(_) => {
            response::not_found()
        },
    }
}






















