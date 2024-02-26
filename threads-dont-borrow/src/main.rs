use std::thread;

/**
 * Threads dont borrow in rust so you need to `move ||`
 */
fn main() {
    let name = "dolly".to_string();
    let t = thread::spawn(move || {
        println!("hello {}", name);
    });
    println!("wait {:?}", t.join());
}