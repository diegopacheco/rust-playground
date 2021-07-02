use std::sync::mpsc::channel;
use std::thread::spawn;

fn main() {
    simple();
    with_threads();
}

fn simple(){
    println!("Simple...");
    let (sender,receiver) = channel();
    sender.send(42);
    sender.send(171);
    println!("{}",receiver.recv().unwrap());

    if let Ok(i) = receiver.recv() {
        println!("Got it {:?}",i);
    } else{
        println!("Oops!");
    }

    println!("{:?}",receiver.try_recv());
}

fn with_threads() {
    println!("with_threads...");
    let mut handlers = vec![];
    let (sender,receiver) = channel();
    let r2 = sender.clone();

    handlers.push(spawn(move ||{
        sender.send(42);
    }));
    handlers.push(spawn(move ||{
        r2.send(171);
    }));

    for h in handlers{
        h.join().unwrap();
    }

    dbg!(receiver.recv());
    dbg!(receiver.try_recv());
}