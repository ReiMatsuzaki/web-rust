use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{BufRead, BufReader, copy, Write, Read};
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
    let path = PathBuf::from(format!("/Users/matsuzakirei/src/github.com/ReiMatsuzaki/web-rust/www{}", path));
    let open_file = File::open(&path);
    match open_file {
        Ok(file) => {
            let len = file.metadata().map(|m| m.len()).unwrap_or(0);

            writeln!(stream, "HTTP/1.1 200 OK").unwrap();
            writeln!(stream, "Content-Type: text/html; charset=UTF-8").unwrap();
            writeln!(stream, "Content-Length: {}", len).unwrap();
            writeln!(stream).unwrap();
            let mut file2 = file; // TODO: this code seems illegal
            copy(&mut file2, stream).unwrap();
        },
        Err(_) => {
            writeln!(stream, "HTTP/1.1 501 ERR").unwrap();
        },
    };
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
        writeln!(stream, "{}: {}", k, v);
    }
    writeln!(stream);
    writeln!(stream, "{}", res.body);
}
fn http_response_ok(file: &mut File) -> HttpResponse {

    let mut body = String::new();
    file.read_to_string(&mut body);

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






















