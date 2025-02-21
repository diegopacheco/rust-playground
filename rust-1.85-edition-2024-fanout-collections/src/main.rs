use std::collections::{LinkedList, VecDeque};
fn main() {
    let (squares, cubes, tesseracts): (Vec<_>, VecDeque<_>, LinkedList<_>) =
        (0i32..10).map(|i| (i * i, i.pow(3), i.pow(4))).collect();
    println!("{squares:?}");
    println!("{cubes:?}");
    println!("{tesseracts:?}");
}