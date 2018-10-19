use reqwest;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    TokenNotFound,
    FileIOError(::std::io::Error),
    NetworkError(reqwest::Error),
    JsonParseError(reqwest::Error),
}

impl From<::std::io::Error> for Error {
    fn from(t: ::std::io::Error) -> Self {
        Error::FileIOError(t)
    }
}

impl From<reqwest::Error> for Error {
    fn from(t: reqwest::Error) -> Self {
        if t.is_serialization() {
            Error::JsonParseError(t)
        } else {
            Error::NetworkError(t)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::TokenNotFound => write!(f, "Github Token not found at expected location"),
            Error::FileIOError(e) => fmt::Display::fmt(e, f),
            Error::NetworkError(e) => fmt::Display::fmt(e, f),
            Error::JsonParseError(e) => fmt::Display::fmt(e, f),
        }
        // write!(f, "({}, {})", self.longitude, self.latitude)
    }
}
