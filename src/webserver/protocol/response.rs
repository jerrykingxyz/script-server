use json::JsonValue;

#[derive(Debug)]
pub enum StatusCode {
    Ok,
    Error,
}

pub trait ToResponseBody {
    fn to_json(&self) -> JsonValue;
}

pub struct Response {
    pub status_code: StatusCode,
    pub body: Box<dyn ToResponseBody>,
}

impl Response {
    #[allow(dead_code)]
    pub fn ok(body: Box<dyn ToResponseBody>) -> Self {
        Self {
            status_code: StatusCode::Ok,
            body,
        }
    }

    pub fn error(body: Box<dyn ToResponseBody>) -> Self {
        Self {
            status_code: StatusCode::Error,
            body,
        }
    }
}
