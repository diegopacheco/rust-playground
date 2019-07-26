pub fn execute(){
    let result = sum_and_product(2,3);
    println!("Tuple Result is: {:?}", result);
    println!("Tuple Result is: {0} {1}", result.0, result.1);

    // destructuring
    let (x,y) = result;
    println!("Tuple Result is: {0} {1}", x,y);
}

fn sum_and_product(x:i32,y:i32) -> (i32,i32){
    (x+y,x*y)
}