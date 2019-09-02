use std::num::ParseIntError;

fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n) => Ok(2 * n),
        Err(err) => Err(err),
    }
}

fn print(str: &str){
    match double_number(str) {
        Ok(n)    => println!("OK: {} = {} * {}",n,n/2,n/2),
        Err(err) => println!("Error: {:?}", err),
    }
}

fn main() {
    print("20");
    print("x1");
}