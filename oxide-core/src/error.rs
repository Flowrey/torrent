use std;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    Message(String),
}

impl From<oxide_bencode::Error> for Error {
    fn from(value: oxide_bencode::Error) -> Self {
        Error::Message(value.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Message(value.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
        }
    }
}

impl std::error::Error for Error {}
