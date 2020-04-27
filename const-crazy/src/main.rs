fn main() {
    println!("Crazy const here - works = {} ", CRAZY);
    let _x:i8 = CRAZY;
    const CRAZY:i8 = 10;
}