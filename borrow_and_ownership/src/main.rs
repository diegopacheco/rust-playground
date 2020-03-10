fn main() {
    moved_values_borrowing_to_fix();
    moving();
}

fn moved_values_borrowing_to_fix(){
    let s = String::from("Hello World");
    // let z = s;
    // borrow of moved value: `s`
    let _z = &s;
    println!("{:?}",s);
}

fn moving(){
    let mut v = Vec::new();
    for i in 1..100{
        v.push(i);
    }
    v = moveme(v);
    // moveme(v);
    // borrow of moved value: `v`
    println!("{:?}", v);
}

// fn moveme(v:Vec<i32>) ->  { .. } 
fn moveme(v:Vec<i32>) -> Vec<i32> {
    println!("{:?}", v);
    return v;
}