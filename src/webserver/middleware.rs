use super::constant::AsyncResult;
use super::context::Context;
use anyhow::Result;

pub trait Middleware {
    fn handle<'c>(&self, _ctx: &'c mut Context) -> AsyncResult<'c> {
        Box::new(async { Ok(()) })
    }
    fn post_handle<'c>(&self, _ctx: &'c mut Context, result: Result<()>) -> AsyncResult<'c> {
        Box::new(async { result })
    }
}
