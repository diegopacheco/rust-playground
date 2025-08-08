use std::fs::File;
use std::io::{Read, Result};

fn read_to_string(p: &str) -> Result<String> {
    let mut f = File::open(p)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn run(){
    match read_to_string("Cargo.toml"){
        Ok(s)=> println!("{}", s.lines().next().unwrap_or("")),
        Err(e)=> println!("{}", e),
    }
    let r: Result<i32> = Ok(1);
    println!("{}", r.unwrap_or(0));
}
