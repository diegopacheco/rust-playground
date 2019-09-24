extern crate iron;

use iron::prelude::*;
use iron::status;
use std::env;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {

        let key = "RUST_SVC_VERSION";
        let result = match env::var_os(key) {
            Some(val) => Ok(Response::with((status::Ok, format!("Hello World! V:{}",val.to_str().unwrap())))),
            None => Ok(Response::with((status::Ok, "Hello World! V:Undefined!"))),
        };

        result
    }

    println!("Running on :8080");
    Iron::new(hello_world).http("0.0.0.0:8080").unwrap();
}
