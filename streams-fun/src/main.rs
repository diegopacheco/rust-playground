extern crate async_std;

use async_std::task;
use async_std::prelude::*;
use async_std::stream;
use std::collections::HashMap;

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

    let a = stream::repeat(1u8).take(10);
    let b = stream::repeat(2u8).take(10);
    let mut nums = a.merge(b);
    while let Some(num) = nums.next().await {
        println!("num: {}", num);
    }

    let a = stream::once(1u8);
    let b = stream::once(0u8);
    let s = a.zip(b);
    let _map:HashMap<u8, u8> = s.collect().await;
    assert_eq!(_map.get(&1), Some(&0u8));


}