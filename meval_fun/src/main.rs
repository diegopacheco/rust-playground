extern crate meval;

fn main() {
    let r = meval::eval_str("1 + 2").unwrap();
    println!("1 + 2 = {}", r);
}