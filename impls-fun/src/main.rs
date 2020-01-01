extern crate impls;
use impls::impls;

fn main() {
    struct T;
    trait Foo {}
    trait Bar {}
    impl Foo for T {}

    assert!(impls!(T: Foo ^ Bar),"Oh no Foo is Bar");
    assert!(impls!(str:  !Sized),"Oh no STR is not Sized!");
    assert!(impls!(&mut u32: !Copy & !Clone),"Oh no MUT implement copy and clone now");
    assert!(impls!(&mut u32: Copy),"Fail - hm mut should not implement copy = looks good.");

    println!("{}","Thats it folks!");
}
