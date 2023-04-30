use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread::sleep;
use std::time::Duration;

static PORT: AtomicI32 = AtomicI32::new(8000);

pub struct Server {
  port: i32,
  process: Child,
}

impl Server {
  pub fn new(token: Option<&str>) -> Self {
    let port = PORT.fetch_add(1, Ordering::SeqCst);
    let addr = format!("127.0.0.1:{}", port);

    let mut args: Vec<&str> = vec![];
    if let Some(token) = token {
      args.append(&mut vec!["-t", token]);
    }
    args.append(&mut vec!["-l", &addr, "./tests/scripts"]);

    let process = Command::new("target/debug/script-server")
      .args(args)
      .spawn()
      .unwrap();

    sleep(Duration::from_millis(500));
    Self { port, process }
  }

  pub fn send_tcp(&self, content: &str) -> String {
    let addr = format!("127.0.0.1:{}", self.port);
    let mut client = TcpStream::connect(addr).unwrap();
    (write!(client, "{content}")).unwrap();

    let mut data = String::new();
    client.read_to_string(&mut data).unwrap();
    data
  }
}

impl Drop for Server {
  fn drop(&mut self) {
    self.process.kill().unwrap();
  }
}
