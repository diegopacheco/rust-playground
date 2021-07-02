use std::rc::Rc;
use std::cell::RefCell;

/*
 * Rc: A single-threaded reference-counting pointer.
 * 'Rc' stands for 'Reference Counted'
 * RefCell: A mutable memory location with dynamically
 * checked borrow rule
 */
fn main() {
    let answer = String::from("Gandalfy");

    let shared_string:Rc<RefCell<String>> =
        Rc::new(RefCell::new(answer));
    println!("main - Owners == {}",Rc::strong_count(&shared_string));

    print_rc_cell(Rc::clone(&shared_string));
    add_dot_toupper(Rc::clone(&shared_string));

    print_rc_cell(Rc::clone(&shared_string));

    println!("main - Owners == {}",Rc::strong_count(&shared_string));
    drop(&shared_string);
}

fn add_dot_toupper(s:Rc<RefCell<String>>){
    println!("add_dot_toupper - Owners == {}",Rc::strong_count(&s));
    s.replace(s.take().to_uppercase() + ".");
}

fn add_stars(s:Rc<RefCell<String>>){
    println!("add_stars - Owners == {}",Rc::strong_count(&s));
    s.replace(s.take().to_uppercase() + "***");
}

fn print_rc_cell(s:Rc<RefCell<String>>){
    println!("print_rc_cell - Owners == {}",Rc::strong_count(&s));
    println!("{:?}",s)
}
