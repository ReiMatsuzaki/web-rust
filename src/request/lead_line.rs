use std::io;
use crate::request::Error;

pub struct LeadLine {
    pub method: String,
    pub path: String,
    pub version: String,
}
impl LeadLine {
    pub fn get(path: String) -> LeadLine {
        LeadLine{
            method: "GET".to_string(),
            path,
            version: "1.1".to_string()}
    }
    pub fn post(path: String) -> LeadLine {
        LeadLine{
            method: "POST".to_string(),
            path,
            version: "1.1".to_string()}
    }
    pub fn to_string(&self) -> String {
        format!("{} {} HTTP/{}", self.method, self.path, self.version)
    }
    fn is_valid_version(ver: &String) -> bool {
        ver.eq("1.1")
    }
    pub fn write<W: io::Write>(&self, w: &mut W) -> Result<(), io::Error> {
        write!(w, "{} {} HTTP/{}", self.method, self.path, self.version)
    }
    pub fn read<R: io::BufRead>(r: &mut R) -> Result<LeadLine, Error> {
        let mut line = String::new();
        if let Err(e) = r.read_line(&mut line) {
            return Err(Error::Io(e))
        };
        let mut params = line.split_whitespace();
        let method = params.next();
        let path = params.next();
        let protocol_ver = params.next();
        let (method, path, version) = match (method, path, protocol_ver) {
            (Some(method), Some(path), Some(protocol_ver)) => {
                let x = protocol_ver.to_string();
                let mut params = x.split("/");
                let protocol = params.next();
                let ver = params.next();
                match (protocol, ver) {
                    (Some("HTTP"), Some(ver)) => {
                        (method.to_string(), path.to_string(), ver.to_string())
                    },
                    _ => {
                        let description = "failed to parse lead line".to_string();
                        return Err(Error::ParseLine{line, description})
                    }
                }
            },
            _ => {
                let description = "failed to parse lead line".to_string();
                return Err(Error::ParseLine{line, description})
            }
        };
        if !LeadLine::is_valid_version(&version) {
            Err(Error::InvalidVersion{version})
        } else {
            Ok(LeadLine{method, path, version})
        }

    }
}

#[test]
fn test_lead_line() {
    let path = "/example.com/a/b";

    let expected = format!("GET {} HTTP/1.1", path);
    let calculated = LeadLine::get(path.to_string()).to_string();
    assert_eq!(expected, calculated);

    let expected = format!("POST {} HTTP/1.1", path);
    let calculated = LeadLine::post(path.to_string()).to_string();
    assert_eq!(expected, calculated);
}
