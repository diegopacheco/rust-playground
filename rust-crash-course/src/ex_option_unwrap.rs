pub fn run() {
    print!(">>> Option\n");

    let a: Option<i32> = Some(10);
    let b: Option<i32> = None;
    let x = a.unwrap();
    println!("{}", x);
    
    let y = b.unwrap_or(0);
    println!("{}", y);
    
    let z = a.map(|v| v * 2).unwrap();
    println!("{}", z);
}
