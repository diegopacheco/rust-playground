use std::thread;
use std::sync::Arc;

struct WrapperString(String);

impl WrapperString {
    fn new(s: &str) -> Arc<WrapperString> {
        Arc::new(WrapperString(s.to_string()))
    }
}

fn main() {
    let mut threads = Vec::new();
    let name = WrapperString::new("dolly");

    for i in 0..10 {
        let tname = name.clone();
        let t = thread::spawn(move || {
            println!("hello {} count {}", tname.0, i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().expect("thread failed");
    }
}