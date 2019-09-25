use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{BufRead, BufReader, Error, copy};
use std::path::PathBuf;
use std::fs::File;

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
    let mut stream = BufReader::new(stream);

    let mut first_line = String::new();
    if let Err(err) = stream.read_line(&mut first_line) {
        panic!("error during receive a line: {}", err);
    }

    let mut params = first_line.split_whitespace();
    let method = params.next();
    let path = params.next();
    match (method, path) {
        (Some("GET"), Some(path)) => {
            get_operation(path, stream.get_mut());
        }
        _ => panic!("failed to parse"),
    }
}

fn get_operation(path: &str, stream: &mut TcpStream) {
    let path = PathBuf::from(format!("./www{}", path));
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => {
            panic!("failed to open {}: {}", path.display(), Error::description(&why))
        },
    };
    let len = file.metadata().map(|m| m.len()).unwrap_or(0);

    writeln!(stream, "HTTP/1.1 200 OK").unwrap();
    writeln!(stream, "Content-Type: text/html; charset=UTF-8").unwrap();
    writeln!(stream, "Content-Length: {}", len).unwrap();
    writeln!(stream).unwrap();

    copy(&mut file, stream).unwrap();

}
