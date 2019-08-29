extern crate glue;
use glue::prelude::*;

fn main() {
    println!("Hello, world!");
    
    if take(1.., is(alphabetic)).test("foobar") {
        println!("One or more alphabetic characters found!");
    }

}
