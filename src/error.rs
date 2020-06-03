use std::error;
use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Chip8Error>;

#[derive(Debug)]
pub enum Chip8Error {
    Io(io::Error),
}

impl fmt::Display for Chip8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            // this is a wrapper, so defer to the underlying types impl of `fmt`
            Self::Io(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Chip8Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Io(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for Chip8Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
