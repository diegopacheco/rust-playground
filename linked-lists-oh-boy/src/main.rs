
mod linkedlist;

fn main() {
    println!("Linked List");
    let mut list = linkedlist::List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.pop();
}
