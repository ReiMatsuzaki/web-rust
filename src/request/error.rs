use std::io;
use std::num::ParseIntError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ParseInt(ParseIntError),
    ParseLine {line: String, description: String},
    InvalidVersion {version: String}
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => write!(f, "IO error: {}", e),
            Error::ParseInt(ref e) => write!(f, "parse int error: {}", e),
            Error::ParseLine{ref line, ref description} => write!(f, "parse line error: {}, line: {}", description, line),
            Error::InvalidVersion{ref version} => write!(f, "invalid version: {}", version),
        }
    }
}