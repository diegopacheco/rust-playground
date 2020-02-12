#[macro_use]
extern crate query_interface;

use query_interface::{Object, ObjectClone};
use std::fmt::Debug;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Foo;

interfaces!(Foo: ObjectClone, Debug, Bar);

trait Bar {
    fn do_something(&self);
}
impl Bar for Foo {
    fn do_something(&self) {
        println!("I'm a Foo!");
    }
}

fn main() {
    let obj = Box::new(Foo) as Box<Object>;
    let obj2 = obj.clone();
    println!("{:?}", obj2);
   
    obj2.query_ref::<Bar>().unwrap().do_something();  // Prints: "I'm a Foo!"
}
