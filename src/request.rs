use std::collections::HashMap;
use std::io::BufRead;
use log::info;
use std::io::Read;
use std::num::ParseIntError;

pub struct Header {
    value: HashMap<String, String>
}

impl Header {
    pub fn new() -> Header {
        let header: HashMap<String, String> = HashMap::new();
        Header {
            value: header
        }
    }
    pub fn insert(&mut self, k: String, v: String) {
        self.value.insert(k, v);
    }
    pub fn content_length(&self) -> Result<usize, ParseIntError> {
        match self.value.get("Content-Length") {
            Some(x) => {
                match x.trim().parse::<usize>() {
                    Ok(x) => Ok(x),
                    Err(e) => panic!("failed. e: {}, value: {}, len: {}", e, x, x.len()),
                }
            }
            None => Ok(0),
        }
    }
}

pub struct Body {
    value: HashMap<String, String>
}

impl Body {
    pub fn new() -> Body {
        let header: HashMap<String, String> = HashMap::new();
        Body {
            value: header
        }
    }
    pub fn to_string(&self) -> String {
        let kvs: Vec<String> = self.value.iter().map(
            |(k, v)| format!("{}={}", k, v)).collect();
        kvs.join("&")
    }
    pub fn insert(&mut self, k: String, v: String) {
        self.value.insert(k, v);
    }
    pub fn get(&self, k: &str) -> &String {
        &self.value[k]
    }
}

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    version: String,
    header: Header,
    pub body: Body,
}
impl HttpRequest {
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        buf = format!("{}{} {} HTTP/{}\n", buf, self.method, self.path, self.version);
        for (k, v) in &self.header.value {
            buf = format!("{}{}: {}\n", buf, k, v);
        }
        buf = format!("{}\n", buf);
        buf = format!("{}{}\n", buf, self.body.to_string());
        buf
    }
    fn read_first_line<R: BufRead>(mut reader: R) -> (String, String, R) {
        info!("read_first_line begin");
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
        (method, path, reader)
    }
    fn read_header<R: BufRead>(mut reader: R) -> (Header, R) {
        info!("HttpRequest::from_stream read header");
        let mut done = false;
        let mut header = Header::new();
        while !done {
            let mut line = String::new();
            if let Err(err) = reader.read_line(&mut line) {
                panic!("error during reading stream: {}", err);
            };
            info!("line: {}", line);
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
        (header, reader)
    }
    fn read_body<R: Read>(mut reader: R, len: usize) -> (Body, R) {
        let mut buf: Vec<u8> = Vec::with_capacity(len);
        buf.resize(len, 0);
        if let Err(e) = reader.read(&mut buf) {
            panic!("error: {}", e);
        }
        let body_str: String = buf.iter().map(|&s| s as char).collect();
        let kvs: Vec<&str> = body_str.split("&").collect();
        let mut body = Body::new();
        for kv in kvs{
            let xs: Vec<&str> = kv.split("=").collect();
            let k = xs[0];
            let v = xs[1];
            body.insert(k.to_string(), v.to_string());
        }
        (body, reader)
    }

    pub fn from_stream<R: BufRead>(reader: R) -> (Result<HttpRequest, Box<dyn std::error::Error>>, R) {
        info!("HttpRequest::from_stream begin");

        let (method, path, reader) = HttpRequest::read_first_line(reader);
        let (header, reader) = HttpRequest::read_header(reader);
        let len = match header.content_length() {
            Ok(len) => len,
            Err(e) => panic!("failed to get content_length: {}", e),
        };
        info!("len: {}", len);
        let (body, reader) = HttpRequest::read_body(reader, len);

        // return
        let req = HttpRequest {
            method,
            path,
            version: "1.1".to_string(),
            header,
            body
        };
        (Ok(req), reader)
    }
}
