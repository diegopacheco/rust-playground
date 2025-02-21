use core::future::Future;
use tokio;
use std::boxed::Box;
use std::pin::Pin;

type BoxFuture<'a> = Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

async fn f(_f: impl for<'a> Fn(&'a u8) -> BoxFuture<'a>) -> i32 {
    42
}

async fn f2(_: impl for<'a> AsyncFn(&'a u8)) -> i32 {
    43
}

#[tokio::main]
async fn main() {
    async fn g(x: &u8) {
        println!("Processing value: {}", x);
    }

    let result1 = f(|x| {
        Box::pin(async move {
            g(x).await
        })
    }).await;

    let result2 = f2(g).await;
    
    println!("Results: {} {}", result1, result2);
}