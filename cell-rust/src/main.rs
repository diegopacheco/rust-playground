use std::cell::Cell;

fn main() {
    let answer = Cell::new(42);
    assert_eq!(answer.get(), 42);

    answer.set(66);
    assert_eq!(answer.get(), 66);

    println!("Execute order {}",answer.get());
}
