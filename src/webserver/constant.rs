use anyhow::Result;
use std::future::Future;

pub type AsyncResult<'a> = Box<dyn Future<Output = Result<()>> + Send + 'a>;

//pub type AsyncResultImpl = impl Future<Output = Result<()>>;
