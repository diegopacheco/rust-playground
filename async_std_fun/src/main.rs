extern crate async_std;
use async_std::task;

async fn say_hi() {
    println!("Hello, world!");
}

//
// more on: https://github.com/async-rs/async-std/tree/master/examples
//
fn main() {
    task::block_on(say_hi())
}