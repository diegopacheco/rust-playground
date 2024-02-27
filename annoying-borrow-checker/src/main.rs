use std::collections::HashMap;

fn main() {
    a1();
}

//
// https://stevedonovan.github.io/rust-gentle-intro/pain-points.html
// dont think this is true anymore.
//
fn a1(){
    let mut m = HashMap::new();
    m.insert("one", 0);
    m.insert("two", 2);

    if let Some(r) = m.get_mut("four") { // <-- mutable borrow of m
        *r = 10;
    } else {
        // this works
        *(m.get_mut("two").unwrap()) = 20;

        m.insert("one", 1); // can't borrow mutably again!
    }
    println!("{:?}",m);
}