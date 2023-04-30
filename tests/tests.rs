mod utils;
use utils::Server;

#[test]
fn feature_test() {
  let server = Server::new(None);
  let data = server.send_tcp("GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
  assert!(data.starts_with("HTTP/1.1 404 Not Found"));

  let data = server.send_tcp("POST / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
  assert!(data.starts_with("HTTP/1.1 403 Forbidden"));

  let data =
    server.send_tcp("POST /no_perm HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
  assert!(data.starts_with("HTTP/1.1 403 Forbidden"));

  let data =
    server.send_tcp("GET /print_str HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
  assert!(data.starts_with("HTTP/1.1 404 Not Found"));

  let data =
    server.send_tcp("POST /print_str HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
  assert!(data.contains("print test string"));

  let data =
        server.send_tcp("POST /print_args HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\nContent-Type: plain/text\r\nContent-Length: 14\r\n\r\narg1\narg2\narg3");
  assert!(data.contains("arg1,arg2,arg3"));

  let data =
    server.send_tcp("POST /throw_error HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
  assert!(data.contains("cat: ./a: No such file or directory"));

  let data = server
    .send_tcp("POST /sub/print_info HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
  assert!(data.contains("sub/print_info"));

  let data = server.send_tcp("POST /../a HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
  assert!(data.starts_with("HTTP/1.1 403 Forbidden"));
}

#[test]
fn token_test() {
  let server = Server::new(Some("123"));
  let data =
    server.send_tcp("GET /print_str HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
  assert!(data.starts_with("HTTP/1.1 401 Unauthorized"));

  let data =
    server.send_tcp("POST /print_str HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
  assert!(data.starts_with("HTTP/1.1 401 Unauthorized"));

  let data = server.send_tcp(
    "GET /print_str HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\nX-ACCESS-TOKEN: 123\r\n\r\n",
  );
  assert!(data.starts_with("HTTP/1.1 404 Not Found"));

  let data =
        server.send_tcp("POST /print_str HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\nX-ACCESS-TOKEN: 123\r\n\r\n");
  assert!(data.contains("print test string"));
}
