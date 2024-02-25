use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;

type BoxResult<T> = Result<T,Box<dyn Error>>;

#[derive(Debug)]
struct MyError {
    details: String
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn run(file: &str) -> BoxResult<i32> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().parse()?)
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

// a test function that returns our error result
fn raises_my_error(yes: bool) -> Result<(),MyError> {
    if yes {
        Err(MyError::new("borked"))
    } else {
        Ok(())
    }
}

impl From<ParseFloatError> for MyError {
    fn from(err: ParseFloatError) -> Self {
        MyError::new(err.to_string().as_str())
    }
}

// and test!
fn parse_f64(s: &str, yes: bool) -> Result<f64,MyError> {
    raises_my_error(yes)?;
    let x: f64 = s.parse()?;
    Ok(x)
}

fn main() {
    println!(" {:?}", parse_f64("42",false));
    println!(" {:?}", parse_f64("42",true));
    println!(" {:?}", parse_f64("?42",false));
    println!("{:?}",run("oops"));
}