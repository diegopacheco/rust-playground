pub fn run() {
    print!(">>> Control flow\n");

    let x = 7;
    if x % 2 == 0 {
        println!("even");
    } else if x % 3 == 0 {
        println!("divisible by three");
    } else {
        println!("odd");
    }
}
