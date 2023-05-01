use clap::Parser;
use std::{io::read_to_string, path::PathBuf};
use tiny_http::{Method, Server};

mod error;
mod utils;
use utils::{executable, has_header, run_command, send_response};

/// Command http server
#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
  /// Listen address
  #[arg(short, long, default_value_t = String::from("0.0.0.0:8000"))]
  pub listen: String,

  /// Access token in header
  #[arg(short, long)]
  pub token: Option<String>,

  /// Scripts dir
  #[arg(value_parser = clap::value_parser!(PathBuf))]
  pub scripts_dir: PathBuf,
}

fn main() {
  let args = Args::parse();

  let server = Server::http(&args.listen).unwrap();

  for mut request in server.incoming_requests() {
    if let Some(token) = &args.token {
      if !has_header(&request, "X-ACCESS-TOKEN", token) {
        send_response(request, 401, "no permission");
        continue;
      }
    }

    let method = request.method();
    if !matches!(method, Method::Post) {
      send_response(request, 404, "no found");
      continue;
    }

    let url = request.url();
    if url.contains("/../") {
      send_response(request, 403, "forbidden");
      continue;
    }

    let script_path = &args.scripts_dir.join(&url[1..]);
    if !executable(script_path) {
      send_response(request, 403, "forbidden");
      continue;
    }

    let params: Vec<String> = match read_to_string(request.as_reader()) {
      Ok(body) => {
        if body.is_empty() {
          vec![]
        } else {
          body.split('\n').map(String::from).collect()
        }
      }
      Err(err) => {
        send_response(request, 500, err.to_string());
        continue;
      }
    };

    match run_command(script_path, params) {
      Ok(data) => send_response(request, 200, data),
      Err(err) => send_response(request, 500, err.message),
    };
  }
}
