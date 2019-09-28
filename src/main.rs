extern crate web_rust;

use std::env;
use std::net::TcpListener;
use std::thread;

use env_logger;
use log::info;

use web_rust::dispatcher::Dispatcher;
use std::net::TcpStream;
use web_rust::request::HttpRequest;
use web_rust::response;

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
    let dispatcher: Dispatcher = Dispatcher{};
    let (result, mut stream) =  HttpRequest::from_stream(stream);
    let res = match result {
        Err(_) => response::internal_server_error(),
        Ok(req) => dispatcher.dispatch(req),
    };
    res.write_stream(&mut stream);
}




















