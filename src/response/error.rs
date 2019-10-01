use std::io;

pub enum HttpResponseError {
    Io(io::Error),
}

