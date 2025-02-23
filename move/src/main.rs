fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let multiplier = 10;
    
    let operate = move |mut input: Vec<i32>| {
        input.iter_mut().for_each(|x| *x *= multiplier);
        input
    };
    
    let result = operate(data);
    println!("Processed data: {:?}", result);
    //println!("{:?}", data); 
                    // ^^^^ value borrowed here after move
}