fn main() {
    let ints = [1,2,3,4,5,6,7,8,9,10];
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window {:?}", s);
    }
    for s in slice.chunks(2) {
        println!("chunks {:?}", s);
    }

    println!("cool range sum {}", (0..5).sum::<i32>());  // inclusive 
    
    let sum: i64 = [0, 1, 2, 3, 4].iter().sum();         // not inclusive  
    println!("cool range sum {sum}");
}