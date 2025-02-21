mod ringbuffer;

fn main() {
    let mut rb = 
    ringbuffer::RingBuffer::new(3);
    
    rb.push(10);
    rb.push(20);
    rb.push(30);
    rb.push(40); // overwrites oldest
   
    println!("Length: {:?}", rb.len());
    println!("Popped: {:?}", rb.pop());
    println!("Length: {:?}", rb.len());
}
