pub fn execute(){
    let add_one = |i:i32| -> i32 { i + 1 };
    println!("Closure in action in Rust 13+1={}", add_one(13));
}