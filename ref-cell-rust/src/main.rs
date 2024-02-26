use std::cell::RefCell;

fn main() {
    let greeting = RefCell::new("Rust".to_string());
    assert_eq!(*greeting.borrow(), "Rust");
    assert_eq!(greeting.borrow().len(), 4);
    print!("Greetings message is: {} \n",greeting.borrow().to_string());

    *greeting.borrow_mut() = "Ola!".to_string();
    assert_eq!(*greeting.borrow(), "Ola!");

    print!("Greetings message is: {} \n",greeting.borrow().to_string());
}
