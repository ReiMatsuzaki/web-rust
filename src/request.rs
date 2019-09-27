use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{BufReader, BufRead};

pub struct HttpRequest {
    method: String,
    path: String,
    version: String,
    header: HashMap<String, String>,
}
pub fn from_stream(stream: TcpStream) -> Result<HttpRequest, Box<std::error::Error>> {
    let mut reader: BufReader<TcpStream> = BufReader::new(stream);

    let mut first_line = String::new();
    reader.read_line(&mut first_line);
    let mut params = first_line.split_whitespace();
    let method = params.next();
    let path = params.next();
    let (method, path) = match (method, path) {
        (Some(method), Some(path)) => (method.to_string(), path.to_string()),
        _ => panic!("failed to get key and path"),
    };

    let mut header: HashMap<String, String> = HashMap::new();
    for result in reader.lines() {
        let mut params = result.unwrap().split_whitespace();
        let key = params.next();
        let value = params.next();
        match (key, value) {
            (Some(key), Some(value)) => header.insert(key.to_string(), value.to_string()),
            _ => panic!("failed to get key and value"),
        }
    }
    Ok(HttpRequest {
        method,
        path,
        version: "1.1".to_string(),
        header
    })
}