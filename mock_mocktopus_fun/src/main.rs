use mocktopus_macros::mockable;

fn main() {
    println!("Hello, world!");
}

#[mockable]
#[allow(dead_code)]
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
    use mocktopus::mocking::MockResult;
    use mocktopus::mocking::*;

    hello_world::world.mock_safe(|| MockResult::Return("mocking"));
    assert_eq!("Hello mocking!", hello_world::hello_world());
}