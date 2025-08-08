pub fn run() {
    print!(">>> Loops\n");

    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("{}", sum);
    
    let v = vec![1,2,3];
    for x in &v { println!("{}", x); }
    for x in v.into_iter().rev() { println!("{}", x); }
}
