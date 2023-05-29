use std::array::TryFromSliceError;
use std::fmt::{self, Display};
use std::{self, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    DeError(serde_bencode::error::Error),
    Message(String),
    BufferSier(TryFromSliceError)
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<serde_bencode::error::Error> for Error {
    fn from(error: serde_bencode::error::Error) -> Self {
        Self::DeError(error)
    }
}

impl From<TryFromSliceError> for Error {
    fn from(error: TryFromSliceError) -> Self {
        Self::BufferSier(error)
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::IOError(e) => formatter.write_str(&e.to_string()),
            Error::DeError(e) => formatter.write_str(&e.to_string()),
            Error::BufferSier(e) => formatter.write_str(&e.to_string())
        }
    }
}

impl std::error::Error for Error {}
