use smol::Async;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use utils::Result;

mod error_handler;
mod protocol;

// use protocol::HttpClient;

const RESPONSE: &[u8] = br#"
HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: 47
<!DOCTYPE html><html><body>Hello!</body></html>
"#;

async fn serve(mut stream: Async<TcpStream>) -> Result<()> {
    let req = http::Request::generate(stream);
    error_handler::error_handler(res);
    stream.write_all(RESPONSE).await?;
    Ok(())
}

pub fn listen() -> Result<()> {
    smol::block_on(async {
        let listener = Async::<TcpListener>::bind(8000)?;
        loop {
            // Accept the next connection.
            let (stream, _) = listener.accept().await?;

            smol::spawn(async move {
                if let Err(err) = serve(stream).await {
                    println!("Connection error: {:#?}", err);
                }
            })
            .detach();
        }
        Ok(())
    })
}
