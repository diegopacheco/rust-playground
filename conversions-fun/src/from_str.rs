use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

impl FromStr for Person {
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        let tokens:Vec<&str>= s.split(",").collect();
        let p:Person = Person{
            name: String::from(tokens[0]),
            age: tokens[1].parse::<usize>().unwrap(),
        };
        return Ok(p);
    }
}

pub fn main_mod() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        assert!("John,32".parse::<Person>().is_ok());
    }
    #[test]
    #[should_panic]
    fn missing_age() {
        "John".parse::<Person>().unwrap();
    }
}