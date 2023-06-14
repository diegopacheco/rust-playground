// extern crate rary; 
// May be required for Rust 2015 edition or earlier
fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();
    rary::indirect_access();
}