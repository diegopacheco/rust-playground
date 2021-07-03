use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct ThreadPool {
    _workers: Vec<Worker>,
    tx: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(worker_count: usize) -> Self {
        let (tx, rx) = mpsc::channel();

        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(worker_count);

        for n in 0..worker_count {
            workers.push(Worker::new(n, Arc::clone(&rx)))
        }

        ThreadPool {
            _workers: workers,
            tx,
        }
    }

    pub fn execute<F>(&self, callback: F)
        where
            F: FnOnce() + Send + 'static,
    {
        self.tx
            .send(Box::new(callback))
            .expect("Thread shut down too early");
    }
}

struct Worker {
    _id: usize,
    _handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let result = rx.lock().unwrap().recv();
            match result {
                Ok(rx) => {
                    println!("Worker {} got a job; executing.", id);
                    rx()
                }
                Err(_) => {
                    println!("Worker {} signing off", id);
                    break;
                }
            }
        });
        Worker {
            _id: id,
            _handle: handle,
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

fn main(){
    let pool = ThreadPool::new(4);
    let count = Arc::new(AtomicUsize::new(0));

    let count1 = count.clone();
    pool.execute(move || {
        println!("Thread 1");
        count1.fetch_add(1, Ordering::SeqCst);
        std::thread::sleep(std::time::Duration::from_secs(1));
    });

    let count2 = count.clone();
    pool.execute(move || {
        println!("Thread 2");
        count2.fetch_add(1, Ordering::SeqCst);
        std::thread::sleep(std::time::Duration::from_secs(1));
    });

    let count1 = count.clone();
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("Count {:?}",&count1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_threadpool() {
        let pool = ThreadPool::new(4);
        let count = Arc::new(AtomicUsize::new(0));

        let count1 = count.clone();
        pool.execute(move || {
            println!("Thread 1");
            count1.fetch_add(1, Ordering::SeqCst);
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
        let count2 = count.clone();
        pool.execute(move || {
            println!("Thread 2");
            count2.fetch_add(1, Ordering::SeqCst);
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
        let count3 = count.clone();
        pool.execute(move || {
            println!("Thread 3");
            count3.fetch_add(1, Ordering::SeqCst);
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
        let count4 = count.clone();
        pool.execute(move || {
            println!("Thread 4");
            count4.fetch_add(1, Ordering::SeqCst);
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
        std::thread::sleep(std::time::Duration::from_secs(2));

        let count = count.load(Ordering::SeqCst);

        assert_eq!(count, 4);
    }
}