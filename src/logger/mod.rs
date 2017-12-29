use nickel::{MiddlewareResult, Middleware, Request, Response};

// Middleware Logger Implementation
pub struct Logger;
impl<D> Middleware<D> for Logger {
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, res: Response<'mw, D>)
    -> MiddlewareResult<'mw, D> {
        println!("logging request from logger middleware: {:?}", req.origin.uri);
        res.next_middleware()
    }
}
