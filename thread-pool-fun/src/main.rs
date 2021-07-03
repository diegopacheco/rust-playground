use std::thread::JoinHandle;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::ops::Deref;

struct ThreadPool{
    handlers:Vec<JoinHandle<()>>,
    sender:Sender<Box<Task>>
}

trait Task{}
impl Task for i32{}
impl Task for String{}

impl ThreadPool {
    fn new(size:u16) -> Self{
        let (sender,receiver) = std::sync::mpsc::channel::<Box<Task>>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut handlers:Vec<JoinHandle<()>> = vec![];

        for _ in 0..size{
            let clone = receiver.clone();
            let handle = std::thread::spawn(move || loop {
                    let task = clone.lock().unwrap().recv().unwrap();
                    task();
            });
            handlers.push(handle);
        }

        Self{
            handlers: handlers,
            sender: sender
        }
    }
    fn submit(self,task:Box<Task>){
        let clone_sender = self.sender.clone();
        clone_sender.send(task);
    }
}

fn main() {
    let tp:ThreadPool = ThreadPool::new(10);
    tp.submit(Box::new(get_task()));
}

fn get_task() -> impl Task {
    println!("Running to deliver 42");
    42
}
