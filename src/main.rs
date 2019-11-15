extern crate nickel;
use std::process::Command;
use nickel::status::StatusCode;
use nickel::{HttpRouter, MiddlewareResult, Nickel, QueryString, Request, Response};
//todo, wait, headers

fn handle<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let query = _req.query();
    if query.get("statuscode").is_some() {
        let statuscode: u16 = query.get("statuscode").unwrap().parse().unwrap();
        println!("Returning {:?} code", statuscode);
        res.set(StatusCode::Unregistered(statuscode).class().default_code());
    }
    if query.get("wait").is_some() {
        let wait = query.get("wait").unwrap();
        println!("Waiting {:?}", wait);
        let mut child = Command::new("sleep").arg(wait).spawn().unwrap();
        let _result = child.wait().unwrap();
    }
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
