extern crate mock_derive;

use mock_derive::mock;

pub fn main(){}

pub struct Foo {}

#[mock]
pub trait CustomTrait {
    fn get_int(&self) -> u32;
    fn opt_int(&self) -> Option<u32>;
    fn default_method(&self, x: i32, y: i32) -> i32 {
        x + y
    }
}

impl CustomTrait for Foo {
    fn get_int(&self) -> u32 {
        return 1;
    }
    fn opt_int(&self) -> Option<u32> {
        return Some(1);
    }
}

#[test]
fn return_result_of() {
    let mut x = 15;
    let mut mock = MockCustomTrait::new();
    let method = mock.method_opt_int()
        .return_result_of(move || {
            x += 1;
            Some(x)
        });

    mock.set_opt_int(method);
    assert!(mock.opt_int() == Some(16));
    assert!(mock.opt_int() == Some(17));
    assert!(mock.opt_int() == Some(18));
}
