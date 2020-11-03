extern crate scipio;
use scipio::LocalExecutorBuilder;

fn main() {

    LocalExecutorBuilder::new().spawn(|| async move {
        println!("1 - Hello, world! Scipio - Thread-per-core fun :D ");
    }).unwrap();

    LocalExecutorBuilder::new().spawn(|| async move {
        println!("2 - Hello, world! Scipio - Thread-per-core fun :D ");
    }).unwrap();

    LocalExecutorBuilder::new().spawn(|| async move {
        println!("3 - Hello, world! Scipio - Thread-per-core fun :D ");
    }).unwrap();
}
