extern crate nickel;
use nickel::status::StatusCode;
use nickel::{HttpRouter, MiddlewareResult, Nickel, QueryString, Request, Response};
use std::{thread, time};

fn handle<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let query = _req.query();
    if query.get("statuscode").is_some() {
        let statuscode: u16 = query.get("statuscode").unwrap().parse().unwrap();
        println!("Returning {:?} code", statuscode);
        res.set(StatusCode::Unregistered(statuscode).class().default_code());
    }
    if query.get("wait").is_some() {
        let wait: u64 = query.get("wait").unwrap().parse().unwrap();
        println!("Waiting {:?}", wait);
        let ten_millis = time::Duration::from_millis(wait);
        thread::sleep(ten_millis);
    }
    res.headers_mut().set_raw("String-Header", vec![b"string-value".to_vec()]);
    res.headers_mut().set_raw("Integer-Header", vec![b"1".to_vec()]);
    res.send("Finished")
}

fn main() {
    let mut server = Nickel::new();
    server
        .get("**", handle)
        .post("**", handle)
        .put("**", handle);
    server.listen("127.0.0.1:6767").unwrap();
}
