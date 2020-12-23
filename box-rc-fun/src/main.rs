use std::rc::Rc;

fn main() {

    // used once - read and write.
    let mut data = Box::new("data");
    println!("{:#?}",&data);
    data = Box::new("new Data");
    println!("{:#?}",&data);

    // read only for any number of users
    let mut ref_count = Rc::new("data");
    println!("{:#?}",&ref_count);
}
