
mod linkedlist;

fn main() {
    println!("Linked List");
    let mut list = linkedlist::List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    
    println!("* 4 == {:?}", list.peek());
    println!("* 4 == {:?}", list.peek_mut());
    println!("* 4 == {:?}", list.pop());
    println!("* 3 == {:?}", list.peek());

    println!("** Print Whole List ");
    for e in list.into_iter() {
        println!("{:?}", e);
    }

}
