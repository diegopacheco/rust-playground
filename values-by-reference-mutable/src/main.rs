fn modifies(x: &mut f64) {
    *x *= 2.0;
}

fn main() {
    let mut res:f64 = 3.0;
    modifies(&mut res);          // 6.0
    println!("res is {}", res);
}