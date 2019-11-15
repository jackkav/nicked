extern crate nickel;

use nickel::status::StatusCode;
use nickel::{HttpRouter, MiddlewareResult, Nickel, QueryString, Request, Response};
//todo, wait, headers

fn handle<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let query = _req.query();
    let statuscode: u16 = query.get("statuscode").unwrap().parse().unwrap();
    println!("Returning {:?} Code", statuscode);
    res.set(StatusCode::Unregistered(statuscode).class().default_code());
    res.send("hello world")
}

fn main() {
    let mut server = Nickel::new();
    server
        .get("**", handle)
        .post("**", handle)
        .put("**", handle);
    server.listen("127.0.0.1:6767").unwrap();
}
