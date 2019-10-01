use crate::request::lead_line::LeadLine;
use crate::request::header::Header;
use crate::request::body::Body;
use std::io::Write;

pub struct HttpRequest {
    pub lead_line: LeadLine,
    pub header: Header,
    pub body: Body,
}

impl HttpRequest{
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        buf = format!("{}{}", buf, self.lead_line.to_string());
        buf = format!("{}{}", buf, self.header.to_string());
        buf = format!("{}\n", buf);
        buf = format!("{}{}\n", buf, self.body.to_string());
        buf
    }
    pub fn write<W: Write>(&self, w: &mut W) {
        write!(w, "{}", self.lead_line.to_string());
        write!(w, "{}", self.header.to_string());
        write!(w, "\n");
        write!(w, "{}\n", self.body.to_string());
    }
}
