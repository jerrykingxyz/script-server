use json::JsonValue;

#[derive(Debug)]
pub enum StatusCode {
    OK,
    ERROR,
}

pub trait ToResponseBody {
    fn to_json(&self) -> JsonValue;
}

pub struct Response {
    pub status_code: StatusCode,
    pub body: Box<dyn ToResponseBody>,
}

impl Response {
    pub fn ok(body: Box<dyn ToResponseBody>) -> Self {
        Self {
            status_code: StatusCode::OK,
            body,
        }
    }

    pub fn error(body: Box<dyn ToResponseBody>) -> Self {
        Self {
            status_code: StatusCode::ERROR,
            body,
        }
    }
}
