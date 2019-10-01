use std::io;

pub enum Error {
    Io(io::Error),
}

