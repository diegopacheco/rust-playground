use std::{borrow::BorrowMut, rc::Rc};

fn main() {
    let s = "Hello world in Rust".to_string();
    let mut rs1 = Rc::new(s);  // s moves to heap; ref count 1
    let rs2 = rs1.clone(); // ref count 2
    println!("len {}, len {}", rs1.len(), rs2.len());
    
    *rs1.borrow_mut() = Rc::new("Rust".to_string());
    println!("RC value {}", rs1);
}
