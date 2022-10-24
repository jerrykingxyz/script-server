// POST https://localhost:8080/saveBookDetails
// BODY: {"id": "123", "name": "book1", "year": "2020"}
use super::{Request, Response, StatusCode};
use crate::utils::{Error, Result};
use json::stringify;
use smol::io::BufReader;
use smol::{prelude::*, Async};
use std::net::TcpStream;

fn parse_function_name(s: &str) -> Result<String> {
    let mut sp = s.split(' ');
    if !matches!(sp.next(), Some("POST")) {
        return Err(Error::new("only allow use POST method".to_string()));
    }

    let path = sp.next().unwrap_or("");
    if !path.starts_with('/') {
        return Err(Error::new("http path should start with /".to_string()));
    }

    if !matches!(sp.next(), Some("HTTP/1.1\r\n")) {
        return Err(Error::new("only support http 1.1".to_string()));
    }

    if sp.next().is_some() {
        return Err(Error::new("http format error".to_string()));
    }

    Ok(path[1..].to_string())
}

pub async fn gen_request(stream: &Async<TcpStream>) -> Result<Request> {
    let mut s = String::new();
    let mut reader = BufReader::new(stream);
    reader.read_line(&mut s).await?;
    let function_name = parse_function_name(&s)?;
    Ok(Request {
        token: String::from(""),
        function_name,
    })
}

pub async fn send_response(mut stream: &Async<TcpStream>, res: &Response) -> Result<()> {
    let body_text = stringify(res.body.to_json());
    let status_text = match res.status_code {
        StatusCode::Ok => "200 OK".to_string(),
        StatusCode::Error => "500 Internal Server Error".to_string(),
    };
    let http_data = format!(
        r#"HTTP/1.1 {}
Content-Type: application/json
Content-Length: {}

{}"#,
        status_text,
        body_text.len(),
        body_text
    );
    println!("response is {:?}", http_data);
    stream.write_all(&http_data.into_bytes()).await?;
    Ok(())
}
