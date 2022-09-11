mod protocol;

use crate::utils::Result;
use json::{object, JsonValue};
use protocol::{gen_request, send_response, Response, StatusCode};
use smol::Async;
use std::net::{TcpListener, TcpStream};

pub use protocol::ToResponseBody;

struct Test {}
impl ToResponseBody for Test {
    fn to_json(&self) -> JsonValue {
        object! {}
    }
}

pub struct App;

impl App {
    pub fn new() -> App {
        App {}
    }

    async fn serve(&self, stream: &Async<TcpStream>) -> Result<Response> {
        let _request = gen_request(stream).await?;
        Ok(Response {
            status_code: StatusCode::OK,
            body: Box::new(Test {}),
        })
    }

    pub fn listen(&self) -> Result<()> {
        smol::block_on(async {
            let listener = Async::<TcpListener>::bind(([127, 0, 0, 1], 8000))?;
            loop {
                // Accept the next connection.
                let (stream, _) = listener.accept().await?;

                //                smol::spawn(async move {
                let result = self.serve(&stream).await;
                //    error_handler::error_handler(res);
                let response = result.unwrap_or_else(|err| Response::error(Box::new(err)));
                send_response(&stream, &response).await?;
                //                })
                //                .detach();
            }
        })
    }
}
