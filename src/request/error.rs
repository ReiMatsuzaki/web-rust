use std::io;
use std::num::ParseIntError;
use std::fmt;

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