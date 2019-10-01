use std::io::BufRead;
use log::info;
use crate::request::{Body, Error, Header, HttpRequest, LeadLine};

pub struct Parser<R: BufRead> {
    pub reader: R
}
impl<R: BufRead> Parser<R> {
    fn parse_lead_line(&mut self) -> Result<LeadLine, Error> {
        info!("read_first_line begin");
        LeadLine::read(&mut self.reader)
    }
    fn parse_header(&mut self) -> Result<Header, Error> {
        info!("HttpRequest::from_stream read header");
        let mut done = false;
        let mut header = Header::new();
        while !done {
            let mut line = String::new();
            if let Err(err) = self.reader.read_line(&mut line) {
                return Err(Error::Io(err))
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
                    return Err(Error::ParseLine{line, description});
                }
            }
        }
        Ok(header)
    }
    fn parse_body(&mut self, len: usize) -> Result<Body, Error> {

        let mut body = Body::new();
        if len > 0 {
            let mut buf: Vec<u8> = Vec::with_capacity(len);
            buf.resize(len, 0);
            if let Err(e) = self.reader.read(&mut buf) {
                return Err(Error::Io(e))
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
                        return Err(Error::ParseLine { line, description });
                    }
                }
            }
        }
        Ok(body)
    }

    pub fn parse_stream(&mut self) -> Result<HttpRequest, Error> {
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
