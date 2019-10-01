extern crate web_rust;

use std::env;
use std::net::TcpListener;
use std::thread;
use std::net::TcpStream;
use std::io::{self, BufReader};

use env_logger;
use log::{info, error};
use web_rust::response::HttpResponse;
use web_rust::request::Parser;
use web_rust::web_server::WebServer;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("server begin");

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
               thread::spawn(move || {
                   handle_client(stream);
               });
            }
            Err(_) => { panic!("connection failed")}
        }
    }
}

fn handle_client(stream: TcpStream) {
    info!("handle begin");
    let mut web_server = WebServer::new();
    let reader: BufReader<TcpStream> = BufReader::new(stream);
    let mut builder = Parser { reader };
    let rep = match builder.parse_stream() {
        Err(e) => {
            error!("{}", e);
            HttpResponse::internal_server_error()
        },
        Ok(req) => {
            info!("request: ");
            println!("{}", req.to_string());
            web_server.response(&req)
        },
    };
    info!("response: ");
    rep.write_stream(&mut io::stdout()).unwrap();
    let mut stream = builder.reader.into_inner();


    match rep.write_stream(&mut stream) {
        Err(e) => {
            error!("{}", e);
        },
        Ok(()) => {
            info!("handle success")
        },
    };
}