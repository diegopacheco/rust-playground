#[macro_use] extern crate lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

#[allow(unused_mut)]
fn insert(fruit: &str) -> Result<(),()> {
    let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard");
    db.unwrap().push(fruit.to_string());
    Ok(())
}

fn show(){
    {
        let db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard");
        db.iter().enumerate().for_each(|(i, item)| println!("{}: {:?}", i, item));
    }
}

fn main() -> Result<(),()> {
    insert("apple").unwrap();
    insert("orange").unwrap();
    insert("peach").unwrap();
    show();
    insert("grape").unwrap();
    show();
    Ok(())
}