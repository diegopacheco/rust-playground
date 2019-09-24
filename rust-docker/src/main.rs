extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    println!("Running on :8080");
    Iron::new(hello_world).http("0.0.0.0:8080").unwrap();
}
