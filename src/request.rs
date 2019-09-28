use std::collections::HashMap;
use std::io::BufRead;
use log::info;
use std::io;
use std::num::ParseIntError;
use std::fmt;

pub struct LeadLine {
    pub method: String,
    pub path: String,
    pub version: String,
}
impl LeadLine {
    pub fn to_string(&self) -> String {
        format!("{} {} HTTP/{}\n", self.method, self.path, self.version)
    }
}

pub struct Header {
    value: HashMap<String, String>,
}

impl Header {
    pub fn new() -> Header {
        let header: HashMap<String, String> = HashMap::new();
        Header {
            value: header,
        }
    }
    fn to_string(&self) -> String {
        let mut buf = String::new();
        for (k, v) in &self.value {
            buf = format!("{}{}: {}\n", buf, k, v);
        };
        buf
    }
    fn insert(&mut self, k: String, v: String) {
        self.value.insert(k, v);
    }
    fn content_length(&self) -> Result<usize, HttpRequestError> {
        match self.value.get("Content-Length") {
            Some(x) => {
                match x.trim().parse::<usize>() {
                    Ok(x) => Ok(x),
                    Err(e) => Err(HttpRequestError::ParseInt(e)),
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
    pub lead_line: LeadLine,
    pub header: Header,
    pub body: Body,
}
impl HttpRequest {
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        buf = format!("{}{}", buf, self.lead_line.to_string());
        buf = format!("{}{}", buf, self.header.to_string());
        buf = format!("{}\n", buf);
        buf = format!("{}{}\n", buf, self.body.to_string());
        buf
    }
}


#[derive(Debug)]
pub enum HttpRequestError {
    Io(io::Error),
    ParseInt(ParseIntError),
    ParseLine {line: String, description: String},
}
impl fmt::Display for HttpRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpRequestError::Io(ref e) => write!(f, "IO error: {}", e),
            HttpRequestError::ParseInt(ref e) => write!(f, "parse int error: {}", e),
            HttpRequestError::ParseLine{ref line, ref description} => write!(f, "parse line error: {}, line: {}", description, line),
        }
    }
}

pub struct HttpRequestParser<R: BufRead> {
    pub reader: R
}
impl<R: BufRead> HttpRequestParser<R> {
    fn parse_lead_line(&mut self) -> Result<LeadLine, HttpRequestError> {
        info!("read_first_line begin");
        let mut first_line = String::new();
        if let Err(err) = self.reader.read_line(&mut first_line) {
            panic!("error during reading stream: {}", err);
        };
        let mut params = first_line.split_whitespace();
        let method = params.next();
        let path = params.next();
        let version = format!("{}", "1.1");
        let (method, path) = match (method, path) {
            (Some(method), Some(path)) => (method.to_string(), path.to_string()),
            _ => panic!("failed to get key and path"),
        };
        Ok(LeadLine{method, path, version})
    }
    fn parse_header(&mut self) -> Result<Header, HttpRequestError> {
        info!("HttpRequest::from_stream read header");
        let mut done = false;
        let mut header = Header::new();
        while !done {
            let mut line = String::new();
            if let Err(err) = self.reader.read_line(&mut line) {
                return Err(HttpRequestError::Io(err))
            };
            info!("line: {}", line);
            if !line.contains(":") {
                done = true;
            } else {
                let sep = ":";
                let params: Vec<&str> = line.split(sep).collect();
                if params.len() > 1 {
                    let key = params[0].to_string();
                    let values: Vec<&str> = params.into_iter().skip(1).collect();
                    let value = values.join(":");
                    header.insert(key, value);
                } else {
                    let description = "invalid format for header".to_string();
                    return Err(HttpRequestError::ParseLine{line, description});
                }
            }
        }
        Ok(header)
    }
    fn parse_body(&mut self, len: usize) -> Result<Body, HttpRequestError> {
        let mut buf: Vec<u8> = Vec::with_capacity(len);
        buf.resize(len, 0);
        if let Err(e) = self.reader.read(&mut buf) {
            return Err(HttpRequestError::Io(e))
        }
        let body_str: String = buf.iter().map(|&s| s as char).collect();
        let kvs: Vec<&str> = body_str.split("&").collect();
        let mut body = Body::new();
        for kv in kvs{
            let xs: Vec<&str> = kv.split("=").collect();
            let k = xs.iter().next();
            let v = xs.iter().next();
            match (k, v) {
                (Some(k), Some(v)) => {
                    body.insert(k.to_string(), v.to_string());
                },
                _ => {
                    let line = body_str;
                    let description = "invalid format for body".to_string();
                    return Err(HttpRequestError::ParseLine{line, description});
                }
            }
        }
        Ok(body)
    }

    pub fn parse_stream(&mut self) -> Result<HttpRequest, HttpRequestError> {
        info!("HttpRequest::from_stream begin");

        let lead_line = match self.parse_lead_line() {
            Ok(x) => x,
            Err(e) => return Err(e),
        };
        let header = match self.parse_header() {
            Ok(x) => x,
            Err(e) => return Err(e),
        };
        let len = match header.content_length() {
            Ok(x) => x,
            Err(e) => return Err(e),
        };
        let body = self.parse_body(len)?;

        // return
        let req = HttpRequest {
            lead_line,
            header,
            body
        };
        Ok(req)
    }
}
