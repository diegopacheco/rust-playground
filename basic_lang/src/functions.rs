pub fn add_one(i : i32) -> i32{
    i + 1
}

pub fn execute(){
    let i = add_one(10);
    println!("addOne: {:?}",i);
    let f:fn(i32)->i32 = add_one;
    println!("addOne: {:?}",f(i));
}
