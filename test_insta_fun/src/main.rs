use insta::assert_debug_snapshot;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_snapshots() {
    let value = vec![1, 2, 3];
    assert_debug_snapshot!(value);
}