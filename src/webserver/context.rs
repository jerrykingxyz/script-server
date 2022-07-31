use http_types::{Request, Response, StatusCode};

pub struct Context {
    pub req: Request,
    pub res: Response,
}

impl Context {
    pub fn new(req: Request) -> Context {
        Context {
            req: req,
            res: Response::new(StatusCode::Ok),
        }
    }
}
