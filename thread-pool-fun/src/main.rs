use std::thread::JoinHandle;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::ops::Deref;
use std::cell::{Cell, RefCell};

struct ThreadPool{
    handlers:Vec<JoinHandle<()>>,
    sender:Sender<Arc<RefCell<Task>>>
}

trait Task{
    fn run(&self){}
}
impl Task for i32{}

impl ThreadPool {
    fn new(size:u16) -> Self{
        let (sender,receiver) = std::sync::mpsc::channel::<Arc<RefCell<Task>>>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut handlers:Vec<JoinHandle<()>> = vec![];

        for _ in 0..size{
            let clone = receiver.clone();
            let handle = std::thread::spawn(move || loop {
                    let task = clone.lock().unwrap().recv().unwrap();
                    task.try_borrow().unwrap().run();
            });
            handlers.push(handle);
        }

        Self{
            handlers: handlers,
            sender: sender
        }
    }
    fn submit(self,task:Arc<RefCell<Task>>){
        let clone_sender = self.sender.clone();
        clone_sender.send(task);
    }
}

fn main() {
    let tp:ThreadPool = ThreadPool::new(10);
    tp.submit(Arc::new(RefCell::new(get_task())));
}

fn get_task() -> impl Task {
    println!("Running to deliver 42");
    42
}
