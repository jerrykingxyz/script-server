use anyhow::Result;
use http_types::{Error, Request, Response, StatusCode};
use smol::Async;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::pin::Pin;

mod constant;
mod context;
mod middleware;

pub use self::constant::AsyncResult;
pub use self::context::Context;
pub use self::middleware::Middleware;

pub struct App {
    middlewares: Vec<Box<dyn Middleware>>,
}

impl App {
    pub fn new() -> App {
        App {
            middlewares: Vec::new(),
        }
    }

    async fn handler_request(&self, req: Request) -> http_types::Result<Response> {
        let mut ctx = Context::new(req);
        //        let mut postHandlerMid: Vec<Box<dyn Middleware>> = Vec::new();
        let mut post_handler_res: Result<()> = Ok(());
        let mut current = 0;
        loop {
            match self.middlewares.get(current) {
                Some(m) => {
                    post_handler_res = Pin::from(m.handle(&mut ctx)).await;
                }
                None => {
                    break;
                }
            };
            if let Err(_) = post_handler_res {
                break;
            }
            current = current + 1;
        }
        loop {
            if current == 0 {
                break;
            }
            current = current - 1;
            match self.middlewares.get(current) {
                Some(m) => {
                    post_handler_res = Pin::from(m.post_handle(&mut ctx, post_handler_res)).await;
                }
                None => {
                    break;
                }
            };
        }
        match post_handler_res {
            Err(err) => Err(Error::new(StatusCode::InternalServerError, err)),
            Ok(_) => Ok(ctx.res),
        }
    }

    pub fn use_middleware(&mut self, mid: Box<dyn Middleware>) {
        self.middlewares.push(mid);
    }
    /*    pub fn listen<A: Into<SocketAddr>>(&self, ipPort: A) -> Result<()> {
            smol::block_on(async {
                // Format the full host address.
                let listener = Async::<TcpListener>::bind(ipPort)?;
                loop {
                    let app = async_dup::Arc::new(self);
                    // Accept the next connection.
                    let (stream, _) = listener.accept().await?;

                    // Spawn a background task serving this connection.
                    let stream = async_dup::Arc::new(stream);
                    let task = smol::spawn(async move {
                        if let Err(err) = async_h1::accept(stream, |req| app.handler_request(req)).await
                        {
                            println!("Connection error: {:#?}", err);
                        }
                    });

                    // Detach the task to let it run in the background.
                    task.detach();
                }
            })
    }*/
    pub fn listen<A: Into<SocketAddr>>(&self, ip_port: A) -> Result<()> {
        smol::block_on(async {
            // Format the full host address.
            let listener = Async::<TcpListener>::bind(ip_port)?;
            loop {
                // Accept the next connection.
                let (stream, _) = listener.accept().await?;
                let stream = async_dup::Arc::new(stream);

                // Spawn a background task serving this connection.
                if let Err(err) = async_h1::accept(stream, |req| self.handler_request(req)).await {
                    println!("Connection error: {:#?}", err);
                }
            }
        })
    }
}
