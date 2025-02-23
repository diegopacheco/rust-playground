const fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        n => n * factorial(n - 1),
    }
}

fn array_map<const N: usize>(arr: &[u32; N]) -> [u32; N] {
    let mut result = [0; N];
    for i in 0..N {
        result[i] = factorial(arr[i]);
    }
    result
}

fn main() {
    const FACT_5: u32 = factorial(5);
    println!("5! = {}", FACT_5);
    
    let numbers = [1, 2, 3, 4, 5];
    let fact_array = array_map(&numbers);
    println!("Factorial array: {:?}", fact_array);
}