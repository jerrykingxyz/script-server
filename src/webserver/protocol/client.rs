use std::net::TcpStream;
use utils::Result;

pub struct Request {
    token: String,
    function: String,
}

pub struct Response {
    status_code: int,
    body: String,
}

pub trait Client {
    fn get_request(stream: TcpStream) -> Result<Request>;
    fn send_response(stream: TcpStream, res: Response) -> Result<()>;
}
