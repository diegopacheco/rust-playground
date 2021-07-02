fn main() {
    println!("dbg!");

    dbg!(vec![1,2,3,4,5,6,7]);

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = dbg!(
            numbers
                .iter()
                .filter(|e|*e%2==0)
                .map(|e|e*2)
                .collect::<Vec<i32>>()
    );

    dbg!(result);
}
