use mocktopus_macros::mockable;
use mocktopus::mocking::MockResult;
use mocktopus::mocking::*;

fn main() {
    println!("Hello, world!");
}

#[mockable]
mod hello_world {
    pub fn world() -> &'static str {
        "world"
    }

    pub fn hello_world() -> String {
        format!("Hello {}!", world())
    }
}

#[test]
fn mock_test() {
    hello_world::world.mock_safe(|| MockResult::Return("mocking"));
    assert_eq!("Hello mocking!", hello_world::hello_world());
}