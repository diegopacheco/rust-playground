use std::future::ready;
use std::vec::Vec;

#[tokio::main]
async fn main() {
    let mut vec: Vec<String> = vec![];

    let fut = async {
        vec.push(ready(String::from("DONE. Async Rust 1.85 - edition 2024")).await);
    };
    fut.await;

    println!("Vector contents: {:?}", vec);
}
