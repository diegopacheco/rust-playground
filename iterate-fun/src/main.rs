#[allow(unused_macros)] extern crate iterate;
#[allow(unused_macros)] use iterate::iterate;

fn main() {
    let mut i = iterate![1, 2, 3];
    for n in 1..4 {
        println!("n=={} iterator={}", n,i.next().unwrap());
    }

    let mut iterator = iterate![1, 2, 3];
    assert_eq!(iterator.next(), Some(1));
    assert_eq!(iterator.next(), Some(2));
    assert_eq!(iterator.next(), Some(3));
    assert_eq!(iterator.next(), None);
}
