use std::cell::RefCell;

fn main() {
    let c = RefCell::new(5);
    let old_value = c.replace_with(|&mut old| old + 1);
    let value = c.into_inner();
    printme(old_value);
    printme(value);
    printme(value);
}

fn printme(value:i32){
    println!("{}",value);
}