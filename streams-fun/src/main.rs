extern crate async_std;

use async_std::task;
use async_std::prelude::*;
use async_std::stream;

fn main() {
    task::block_on(run())
}

async fn run() {
    let mut a = stream::repeat(1u8);
    while let Some(num) = a.next().await {
        println!("A: {:?}", num);
        if num==1{
            break;
        }
    }

    let _b = stream::repeat(1u8)
        .for_each(|num| println!("B: {:?}", num))
        .await;
}