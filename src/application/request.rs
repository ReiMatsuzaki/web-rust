use std::io::BufRead;

use log::info;

use crate::request::body::Body;
use crate::request::error::HttpRequestError;
use crate::request::header::Header;
use crate::request::http_request::HttpRequest;
use crate::request::lead_line::LeadLine;

pub struct HttpRequestParser<R: BufRead> {
    pub reader: R
}
impl<R: BufRead> HttpRequestParser<R> {
    fn parse_lead_line(&mut self) -> Result<LeadLine, HttpRequestError> {
        info!("read_first_line begin");
        let mut line = String::new();
        if let Err(err) = self.reader.read_line(&mut line) {
            panic!("error during reading stream: {}", err);
        };
        let mut params = line.split_whitespace();
        let method = params.next();
        let path = params.next();
        let version = format!("{}", "1.1");
        let (method, path) = match (method, path) {
            (Some(method), Some(path)) => (method.to_string(), path.to_string()),
            _ => {
                let description = "failed to parse lead line".to_string();
                return Err(HttpRequestError::ParseLine{line, description})
            }
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
                    let value = values.join(":").trim().to_string();
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

        let mut body = Body::new();
        if len > 0 {
            let mut buf: Vec<u8> = Vec::with_capacity(len);
            buf.resize(len, 0);
            if let Err(e) = self.reader.read(&mut buf) {
                return Err(HttpRequestError::Io(e))
            }
            let body_str: String = buf.iter().map(|&s| s as char).collect();
            let kvs: Vec<&str> = body_str.split("&").collect();

            for kv in kvs {
                let xs: Vec<&str> = kv.split("=").collect();
                let mut xs = xs.iter();
                let k = xs.next();
                let v = xs.next();
                match (k, v) {
                    (Some(k), Some(v)) => {
                        body.insert(k.to_string(), v.to_string());
                    },
                    _ => {
                        let line = body_str;
                        let description = "invalid format for body".to_string();
                        return Err(HttpRequestError::ParseLine { line, description });
                    }
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
