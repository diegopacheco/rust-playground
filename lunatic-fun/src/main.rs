use lunatic::{Channel, Process};

fn main() {
    let channel = Channel::new(0);
    let vec: Vec<i32> = (0..1_000).collect();

    for i in vec.iter() {
        Process::spawn((*i, vec.clone(), channel.clone()), child).unwrap();
    }

    for _ in vec.iter() {
        let (i, sum) = channel.receive();
        println!("Sum until {}: {}", i, sum);
    }
}

// Child process calculates the sum of numbers of context.1 until context.0 index.
fn child(context: (i32, Vec<i32>, Channel<(i32, i32)>)) {
    let i = context.0;
    let vec = context.1;
    let channel = context.2;
    let sum_until_i: i32 = vec[..=i as usize].iter().sum();
    channel.send((i, sum_until_i));
}