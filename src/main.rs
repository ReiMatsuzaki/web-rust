use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listner.incoming() {
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
fn handle_client(stream: TcpStream) {

}
