use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{BufReader, BufRead};

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    version: String,
    header: HashMap<String, String>,
    body: String,
}
impl HttpRequest {
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        buf = format!("{}{} {} HTTP/{}\n", buf, self.method, self.path, self.version);
        for (k, v) in &self.header {
            buf = format!("{}{}: {}\n", buf, k, v);
        }
        buf = format!("{}\n", buf);
        buf = format!("{}{}\n", buf, self.body);
        buf
    }
}
pub fn from_stream(tcp_stream: TcpStream) -> (Result<HttpRequest, Box<dyn std::error::Error>>, TcpStream) {
    let mut reader: BufReader<TcpStream> = BufReader::new(tcp_stream);

    // method and path
    let mut first_line = String::new();
    if let Err(err) = reader.read_line(&mut first_line) {
        panic!("error during reading stream: {}", err);
    };
    let mut params = first_line.split_whitespace();
    let method = params.next();
    let path = params.next();
    let (method, path) = match (method, path) {
        (Some(method), Some(path)) => (method.to_string(), path.to_string()),
        _ => panic!("failed to get key and path"),
    };


    // header
    let mut done = false;
    let mut header: HashMap<String, String> = HashMap::new();
    while !done {
        let mut line = String::new();
        if let Err(err) = reader.read_line(&mut line) {
            panic!("error during reading stream: {}", err);
        };
        if !line.contains(":") {
            done = true;
        } else {
            let params: Vec<&str> = line.split(':').collect();
            if params.len() > 1 {
                let key = params[0].to_string();
                let values: Vec<&str> = params.into_iter().skip(1).collect();
                let value = values.join(":");
                header.insert(key, value);
            } else {
                panic!("failed to get key and value. line: {}", line)
            }
        }
    }

    let mut done = false;
    let mut body = String::new();
    while !done {
        let mut line = String::new();
        if let Err(err) = reader.read_line(&mut line) {
            panic!("error during reading stream: {}", err);
        };
        if !line.contains(":") {
            done = true;
        } else {
            body = format!("{}{}", body, line);
        }
    }

    // return
    let req = HttpRequest {
        method,
        path,
        version: "1.1".to_string(),
        header,
        body
    };
    let tcp_stream = reader.into_inner();
    (Ok(req), tcp_stream)
}