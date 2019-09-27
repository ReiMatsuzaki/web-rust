use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{BufReader, BufRead};
use std::io::Read;

pub struct HttpRequest {
    method: String,
    pub path: String,
    version: String,
    header: HashMap<String, String>,
}
pub fn from_stream(tcp_stream: TcpStream) -> (Result<HttpRequest, Box<std::error::Error>>, TcpStream) {
    let mut reader: BufReader<TcpStream> = BufReader::new(tcp_stream);

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

    let mut header: HashMap<String, String> = HashMap::new();
    let mut lines_string = String::new();
    if let Err(err) = reader.read_to_string(&mut lines_string) {
        panic!("error during reading stream: {}", err);
    };

    let lines: Vec<&str> = lines_string.split('\n').collect();
    for line in lines {
        let mut params = line.split_whitespace();
        let key = params.next();
        let value = params.next();
        match (key, value) {
            (Some(key), Some(value)) => header.insert(key.to_string(), value.to_string()),
            (_, _) => None,
        };
    }
    let req = HttpRequest {
        method,
        path,
        version: "1.1".to_string(),
        header
    };
    let tcp_stream = reader.into_inner();
    (Ok(req), tcp_stream)
}