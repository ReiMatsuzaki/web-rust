use std::collections::HashMap;
use crate::request::error::Error;

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
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        for (k, v) in &self.value {
            buf = format!("{}{}: {}\n", buf, k, v);
        };
        buf
    }
    pub fn insert(&mut self, k: String, v: String) {
        self.value.insert(k, v);
    }
    pub fn get(&self, k: &str) -> Option<&String> {
        self.value.get(k)
    }
    pub fn content_length(&self) -> Result<usize, Error> {
        match self.value.get("Content-Length") {
            Some(x) => {
                match x.trim().parse::<usize>() {
                    Ok(x) => Ok(x),
                    Err(e) => Err(Error::ParseInt(e)),
                }
            }
            None => Ok(0),
        }
    }
}