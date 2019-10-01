use crate::request::lead_line::LeadLine;
use crate::request::header::Header;
use crate::request::body::Body;

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