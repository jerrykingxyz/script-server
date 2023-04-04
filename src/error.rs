use std::{io::Error as IoError, string::FromUtf8Error};

#[derive(Debug)]
pub struct Error {
  pub message: String,
}

impl Error {
  pub fn new(message: String) -> Error {
    Error { message }
  }
}

impl From<String> for Error {
  fn from(s: String) -> Self {
    Self::new(s)
  }
}

impl From<&str> for Error {
  fn from(s: &str) -> Self {
    Self::new(String::from(s))
  }
}

impl From<IoError> for Error {
  fn from(err: IoError) -> Self {
    Self::new(err.to_string())
  }
}

impl From<FromUtf8Error> for Error {
  fn from(_err: FromUtf8Error) -> Self {
    Self::new("transform char to utf8 error".into())
  }
}

pub type Result<T> = std::result::Result<T, Error>;
