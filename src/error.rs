use std::fmt;
use std::io;
use std::io::Error as IOError;
use thiserror::Error;
pub type JResult<T> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unexpected: {0}, {1}")]
    UnexpectIO(String, io::Error),
    #[error("Unexpected: {0}")]
    Unexpected(String),
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error::Unexpected(e.to_string())
    }
}

impl From<(&str, io::Error)> for Error {
    fn from(e: (&str, io::Error)) -> Self {
        Error::UnexpectIO(e.0.to_string(), e.1)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Unexpected(e)
    }
}

impl From<IOError> for Error {
    fn from(e: IOError) -> Self {
        Error::Unexpected(e.to_string())
    }
}

impl From<Error> for String {
    fn from(e: Error) -> Self {
        format!("{}", e)
    }
}
