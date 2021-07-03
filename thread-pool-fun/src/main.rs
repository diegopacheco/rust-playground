use std::thread::JoinHandle;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex};

struct ThreadPool<T>{
    handlers:Vec<JoinHandle<()>>,
    sender:Sender<fn()->T>,
    receiver:Receiver<fn()->T>,
}

impl <T>ThreadPool<T> {
    fn new(size:u16) -> Self{
        let (sender,receiver) = std::sync::mpsc::channel::<fn()->T>();
        let rec = Arc::new(Mutex::new(&receiver));
        let mut handlers:Vec<JoinHandle<()>> = vec![];

        for _ in 0..size{
            let clone = rec.clone();
            let handle = std::thread::spawn(move || loop {
                    let task = clone.lock().unwrap().try_recv().unwrap();
                    task();
            });
            handlers.push(handle);
        }

        Self{
            handlers: handlers,
            sender: sender,
            receiver: receiver
        }
    }
    fn submit(self,task:(fn()->T)){
        let clone_sender = self.sender.clone();
        clone_sender.send(task);
    }
}

fn main() {
    let tp:ThreadPool<i32> = ThreadPool::new(10);
    let task:(fn()->i32) = || -> i32 {
        println!("Running to deliver 42");
        42
    };
    tp.submit(task);
}
