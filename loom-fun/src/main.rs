use loom::sync::Arc;
use loom::sync::atomic::AtomicUsize;
use loom::sync::atomic::Ordering::{Acquire, Release, Relaxed};
use loom::thread;
use loom::thread::JoinHandle;

#[test]
#[should_panic]
fn buggy_concurrent_inc() {
    loom::model(|| {
        let num = Arc::new(AtomicUsize::new(0));

        let ths: Vec<_> = (0..2)
            .map(|_| {
                let num = num.clone();
                let handler = thread::spawn(move || {
                    let curr = num.load(Acquire);
                    num.store(curr + 1, Release);
                });
                return handler;
            })
            .collect();

        for th in ths {
            th.join().unwrap();
        }

        assert_eq!(2, num.load(Relaxed));
    });
}

fn main() {
    println!("Hello, world!");
}
