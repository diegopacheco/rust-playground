use anyhow::anyhow;
use std::error::Error as StdError;

fn main() {
    println!("Hello, world!i AnyHow!!! ");

    let error = Box::<dyn StdError + Send + Sync>::from("oh no!");
    assert_eq!("oh no!", error.to_string());

    let error = anyhow!(error);
    assert_eq!("oh no!",
            error
             .downcast_ref::<Box<dyn StdError + Send + Sync>>()
             .unwrap()
             .to_string()
    );

    let error2 = anyhow!("oh no!").context("it failed");
    assert_eq!("oh no!", error2.source().unwrap().to_string());
}
