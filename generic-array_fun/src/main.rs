#[macro_use] extern crate generic_array;

fn main() {
    let array = arr![char; 'a', 'b', 'c'];
    assert_eq!(array.into_iter().as_slice(), &['a', 'b', 'c']);
    for i in 0..array.len() {
        println!("{}",array[i]);
    }
}
