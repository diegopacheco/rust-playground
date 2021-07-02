use std::time::{Duration};
use std::time::{SystemTime};

fn main() {
    run_my_dummy_threads();
    run_wait();
    get_sync_result();
    run_async();
    sleepy();
}

fn run_my_dummy_threads(){
    std::thread::spawn(||{
        println!("Running from Thread 1");
    });
    std::thread::spawn(||{
        println!("Running from Thread 2");
    });
    for _ in 0..10_000 {} // Make main Thread busy to let all threads to run.
}

fn run_wait(){
    for i in 0..10{
        let handler = std::thread::spawn(move ||{
            println!("Running from Thread {} - {}*{}=={}",i,2,i,2*i);
        });
        handler.join();
    }
}

fn run_async(){
    let mut handlers = vec![];
    for i in 0..10{
        let handler = std::thread::spawn(move ||{
            println!("Running from Async Thread {} - {}*{}=={}",i,2,i,2*i);
        });
        handlers.push(handler);
    }
    for h in handlers{
        h.join().unwrap();
    }
}


fn get_sync_result() -> i32{
    let computation = std::thread::spawn(|| {
        42
    });
    let result = computation.join().unwrap();
    println!("{}", &result);
    result
}

fn sleepy() -> i32{
    let start = SystemTime::now();
    println!("Before run {:?}", &start);
    let computation = std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(2));
        42
    });
    let result = computation.join().unwrap();
    let end = SystemTime::now();
    let diff = end.duration_since(start).unwrap();
    println!("After run {:?} {}", diff,&result);
    result
}