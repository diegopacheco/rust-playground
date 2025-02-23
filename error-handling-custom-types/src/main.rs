#[derive(Debug)]
enum CustomError {
    InvalidInput(String),
    OutOfBounds { value: i32, max: i32 },
}

fn validate_number(n: i32) -> Result<i32, CustomError> {
    if n < 0 {
        return Err(CustomError::InvalidInput("Number cannot be negative".to_string()));
    }
    if n > 100 {
        return Err(CustomError::OutOfBounds { value: n, max: 100 });
    }
    Ok(n * 2)
}

fn main() {
    let numbers = vec![-1, 50, 150];
    for num in numbers {
        match validate_number(num) {
            Ok(result) => println!("Validated number: {}", result),
            Err(CustomError::InvalidInput(msg)) => println!("Error: {}", msg),
            Err(CustomError::OutOfBounds { value, max }) => {
                println!("Error: {} exceeds maximum value of {}", value, max)
            }
        }
    }
}