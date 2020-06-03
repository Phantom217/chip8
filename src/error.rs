use std::error;
use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Chip8Error>;

#[derive(Debug)]
pub enum Chip8Error {
    Io(io::Error),
}

impl error::Error for Chip8Error {}

impl From<io::Error> for Chip8Error {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl fmt::Display for Chip8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Io(ref e) => e.fmt(f),
        }
    }
}
