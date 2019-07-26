pub fn execute(){
    let x = 10;
    println!("x: {}", x);

    let (x, y) = (1, 2);
    println!("x: {}, y: {}", x,y);

    let mut z = 10;
    println!("Z is muutable: {}",z);
    z = 30;
    println!("Z is muutable: {}",z);
}
