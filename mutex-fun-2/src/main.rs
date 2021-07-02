#![feature(mutex_unlock)]
use std::sync::Mutex;

#[derive(Debug)]
struct Tweet<'a>{
    id: Mutex<i32>,
    content: Mutex<&'a str>,
    author: Mutex<&'a str>,
}

fn get_first_tweet<'a>() -> Tweet<'a> {
    Tweet{
        id: Mutex::new(1),
        author: Mutex::new("Diego"),
        content: Mutex::new("Rust and Mutex are cool"),
    }
}

fn main() {
    let m = Mutex::new(42);
    println!("{:?}",&m);

    let mut value = m.lock().unwrap();
    println!("{:?}",&m);
    println!("{:?}",&value);

    *value=171;
    println!("{:?}",&m);
    println!("{:?}",&value);

    drop(value);
    println!("{:?}",&m);

    let my_tweet = get_first_tweet();
    println!("{:?}",&my_tweet);

    *my_tweet.id.lock().unwrap()=2;
    println!("{:?}",&my_tweet);

    let lock = my_tweet.content.lock();
    if let Ok(mut try_mutex) = my_tweet.content.try_lock() {
        *try_mutex = "hahaha hijaked? really?";
    }else{
        println!("Could not get lock");
    }

    println!("{:?}",&my_tweet);
    Mutex::unlock(lock.unwrap());
    println!("{:?}",&my_tweet);

}
