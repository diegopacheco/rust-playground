fn main() {
    let cant_change:f64 = 3.14;
    println!("{}", cant_change);  // 3.14

    //  compiler error
    // cant_change = 4.5;
    // error[E0384]: cannot assign twice to immutable variable `cant_change`

    let mut can_change;    // i32
    can_change = 42;
    can_change += 1;
    println!("{}", can_change); // 42
}
