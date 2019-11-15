#[macro_use]
extern crate nickel;

use nickel::status::StatusCode;
use nickel::{HttpRouter, Nickel};

fn main() {
    let mut server = Nickel::new();
    server.get(
        "/:statuscode",
        middleware! { |request|
            println!("statuscode: {:?}",request.param("statuscode").unwrap());
            let my_string = request.param("statuscode").unwrap();
            let my_int: u16 = my_string.parse().unwrap();
            StatusCode::Unregistered(my_int).class().default_code()
        },
    );
    // server.get("**", middleware!("Hello World"));
    server.listen("127.0.0.1:6767").unwrap();
}
