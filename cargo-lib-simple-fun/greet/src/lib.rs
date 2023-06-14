pub fn greet_str(message: &str) -> bool {
    return greet(String::from(message));    
}

pub fn greet(message: String) -> bool {
    println!("{}",message);
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_string() {
        let result = greet(String::from("whats up"));
        assert_eq!(true,result);
    }

    #[test]
    fn test_greet_str() {
        let result = greet_str("whats up");
        assert_eq!(true,result);
    }
}
