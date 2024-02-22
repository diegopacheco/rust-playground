fn by_ref(x: &i32) -> i32{
    *x + 1
}

fn main() {
    let i = 10;
    let res1 = by_ref(&i);         // 11
    let res2 = by_ref(&41);        // 42
    println!("{} {}", res1,res2);
}