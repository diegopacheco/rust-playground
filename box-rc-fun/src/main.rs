use std::rc::Rc;

fn main() {

    // used once - read and write. Exclusive Ownership
    let mut data = Box::new("data");
    println!("BOX {:#?}",&data);
    *data = "new Data";
    println!("BOX {:#?}",&data);

    // read only for any number of users - Shared Ownership
    let mut ref_count = Rc::new("data");
    println!("RC {:#?}",&ref_count);
    //*ref_count = "new Data"; // error - cannot assign
}
