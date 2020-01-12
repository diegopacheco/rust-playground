extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures_cpupool::CpuPool;

fn long_running_future(a: u32) -> Box<dyn futures::future::Future<Item = u32, Error = ()> + Send> {
    Box::new(futures::future::result(Ok(a)))
}

fn main() {

    // Create a worker thread pool with four threads
    let pool = CpuPool::new(4);

    // Execute some work on the thread pool, optionally closing over data.
    let a = pool.spawn(long_running_future(2));
    let b = pool.spawn(long_running_future(100));

    // Express some further computation once the work is completed on the thread
    // pool.
    let c = a.join(b).map(|(a, b)| a + b).wait().unwrap();

    // Print out the result
    println!("{:?}", c);
}