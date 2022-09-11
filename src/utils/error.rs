use crate::webserver::ToResponseBody;
use json::{object, JsonValue};
use std::convert::From;

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Error {
        Error { message }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl ToResponseBody for Error {
    fn to_json(&self) -> JsonValue {
        object! {
            message: self.message.clone()
        }
    }
}
