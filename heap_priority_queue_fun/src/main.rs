extern crate priority_queue;

use priority_queue::PriorityQueue;

fn main() {
    let mut pq = PriorityQueue::new();

    assert!(pq.is_empty());
    pq.push("Apples", 5);
    pq.push("Bananas", 8);
    pq.push("Strawberries", 23);
    assert_eq!(pq.peek(), Some((&"Strawberries", &23)));

    println!("Get from: {:?}",pq.get(&"Apples").unwrap());
    println!("For each... ");
    for (item, _) in pq.into_sorted_iter() {
        println!("{}", item);
    }
}