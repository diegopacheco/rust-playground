pub fn execute(){

    let sum = (0..).
        map(|x| x*x).
        take_while(|&x| x < 500).
        filter(|x| x%2==0 ).
        fold(0, |sum, x| sum + x);
    println!("High Order Functions in Rsut == {}", sum);

    fn is_even(n:i32) -> 
    impl Fn() -> bool {
        move || n%2==0
    }
    println!("High Order Function is Rust = {}",is_even(2)());

}