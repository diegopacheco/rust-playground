// values is a slice of i32
fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

fn main() {
    // array
    let arr:[i32; 4] = [10,20,30,40];
    
    // & make the array an slice
    let res:i32 = sum(&arr);
    println!("sum {}", res);
}