extern crate web_rust;

use std::env;
use std::net::TcpListener;
use std::thread;

use env_logger;
use log::{info, error};

use web_rust::dispatcher::Dispatcher;
use std::net::TcpStream;
use web_rust::request::HttpRequestParser;
use web_rust::response;
use std::io::{self, BufReader};

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
    let dispatcher: Dispatcher = Dispatcher {};
    let reader: BufReader<TcpStream> = BufReader::new(stream);

    let mut builder = HttpRequestParser { reader };
    let rep = match builder.parse_stream() {
        Err(e) => {
            error!("{}", e);
            response::internal_server_error()
        },
        Ok(req) => {
            info!("request: ");
            println!("{}", req.to_string());
            dispatcher.dispatch(req)
        },
    };
    info!("response: ");
    rep.write_stream(&mut io::stdout()).unwrap();

    let reader = builder.reader;
    let mut stream = reader.into_inner();
    match rep.write_stream(&mut stream) {
        Err(e) => {
            error!("{}", e);
        },
        Ok(()) => {
            info!("handle success")
        },
    };
}