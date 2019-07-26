pub fn execute(){
    let a:[i32;5] = [1,2,3,4,5];
    println!("arrays: a = {} size", a.len());

    println!("Print all array in rust is easy: {:?}", a);

    let b:[i32;5] = [1,2,3,4,5];
    if a == b && b == [1,2,3,4,5]{
        println!("All array match easy in Rust.")
    }

    let c = [1; 10]; // backfill the aray with all elements going to 10.
    for i in 0..c.len(){
        println!("{}",c[i]);
    }

}