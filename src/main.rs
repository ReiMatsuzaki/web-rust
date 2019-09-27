use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{BufRead, BufReader, Write, Read};
use std::path::PathBuf;
use std::fs::File;
use std::collections::HashMap;

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
        _ => http_response_error()
    };

    let mut tcp_stream = stream.into_inner();
    write_http_response(&res, &mut tcp_stream);
}

fn get_operation(path: &str) -> HttpResponse {
    let path = PathBuf::from(format!("/Users/matsuzakirei/src/github.com/ReiMatsuzaki/web-rust/www{}", path));
    let open_file = File::open(&path);
    match open_file {
        Ok(file) => {
            let mut body = String::new();
            let mut file2 = file; // TODO: this code seems illegal
            match file2.read_to_string(&mut body) {
                Ok(_) => http_response_ok(body),
                Err(_) => http_response_error()
            }
        },
        Err(_) => {
            http_response_error()
        },
    }
}

struct HttpResponse {
    version: String,
    code: i32,
    key_values: HashMap<String, String>,
    body: String
}
fn write_http_response(res: &HttpResponse, stream: &mut TcpStream) {
    let msg = match res.code {
        200 => "OK",
        501 => "Error",
        _ => { panic!("unsupported code") }
    };
    let line = format!("HTTP/{} {} {}", res.version, res.code, msg);
    writeln!(stream, "{}", &line).unwrap();
    for (k, v) in &res.key_values {
        writeln!(stream, "{}: {}", k, v).unwrap();
    }
    writeln!(stream).unwrap();
    writeln!(stream, "{}", res.body).unwrap();
}
fn http_response_ok(body: String) -> HttpResponse {

    let len = format!("{}", body.len().to_string());

    let mut key_values = HashMap::new();
    key_values.insert("Content-Type".to_string(), "text/html; charset=UTF-8".to_string());
    key_values.insert("Content-Length".to_string(), len);

    return HttpResponse{
        version: "1.1".to_string(),
        code: 200,
        key_values: key_values,
        body};
}
fn http_response_error() -> HttpResponse {
    let key_values = HashMap::new();
    let body = String::new();
    HttpResponse{
        version: "1.1".to_string(),
        code: 501,
        key_values: key_values,
        body
    }
}






















