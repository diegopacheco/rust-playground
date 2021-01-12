use glommio::prelude::*;
use std::{thread, time};
use std::fmt::Error;

fn main() {
    let result = LocalExecutorBuilder::new().spawn(|| async move {
        println!("Glomio thread-per-core!");
        return Ok::<String, Error>(String::from("ok"));
    });

    let five_secs = time::Duration::from_secs(5);
    thread::sleep(five_secs);

    let r = result.unwrap();
    println!("{:?}",r);
}
