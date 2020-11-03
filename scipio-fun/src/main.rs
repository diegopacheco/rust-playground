extern crate scipio;
use scipio::LocalExecutorBuilder;
use std::{thread, time};

fn main() {

    LocalExecutorBuilder::new().spawn(|| async move {
        println!("1 - Hello, world! Scipio - Thread-per-core fun :D ");
    }).unwrap();

    let secs = time::Duration::from_secs(3);
    thread::sleep(secs);
    println!("FIN.")
}
