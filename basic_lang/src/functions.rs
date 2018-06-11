pub fn addOne(i : i32) -> i32{
    i + 1
}

pub fn execute(){
    let i = addOne(10);
    println!("addOne: {:?}",i);
    let f:fn(i32)->i32 = addOne;
    println!("addOne: {:?}",f(i));
}
