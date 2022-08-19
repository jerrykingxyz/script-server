// POST https://localhost:8080/saveBookDetails
// BODY: {"id": "123", "name": "book1", "year": "2020"}
use super::client::{Client, Request, Response};
use utils::Result;

pub struct HttpClient;

impl Client for HttpClient {
    fn get_request(stream: TcpStream) -> Result<Request> {
        Ok(Request {
            token: "",
            function_name: "",
        })
    }

    fn send_response(stream: TcpStream, res: Response) -> Result<()> {
        Ok(())
    }
}
