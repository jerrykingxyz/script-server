use crate::error::Result;
use std::{path::Path, process::Command};
use tiny_http::{Request, Response};

pub fn has_header(req: &Request, key: &str, value: &str) -> bool {
  for item in req.headers() {
    if item.field.as_str() == key && item.value.as_str() == value {
      return true;
    }
  }

  false
}

// All files are executable on Windows, so just check is file
#[cfg(windows)]
pub fn executable(file_path: &Path) -> bool {
  match file_path.metadata() {
    Ok(data) => data.is_file(),
    Err(_) => false,
  }
}
#[cfg(unix)]
pub fn executable(file_path: &Path) -> bool {
  use std::os::unix::prelude::*;
  match file_path.metadata() {
    Ok(data) => data.is_file() && data.mode() & 0o111 != 0,
    Err(_) => false,
  }
}

pub fn run_command(script_path: &Path, params: Vec<String>) -> Result<String> {
  let output = Command::new(script_path).args(params).output()?;

  if !output.status.success() {
    return Err(String::from_utf8(output.stderr)?.into());
  }
  Ok(String::from_utf8(output.stdout)?)
}

pub fn send_response<S>(request: Request, status_code: i32, body: S)
where
  S: Into<String>,
{
  let result = request.respond(Response::from_string(body).with_status_code(status_code));
  if let Err(err) = result {
    println!("send response with error {err:?}");
  }
}
