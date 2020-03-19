#[macro_use]
extern crate throw;

use std::io::prelude::*;
use std::io;
use std::fs::File;

fn read_log() -> Result<String, throw::Error<io::Error>> {
    let mut file = throw!(File::open("some_file.log"));
    let mut buf = String::new();
    throw!(file.read_to_string(&mut buf));
    Ok(buf)
}

fn do_things() -> Result<(), throw::Error<io::Error>> {
    let log_contents = up!(read_log());
    println!("Log contents: {}", log_contents);
    Ok(())
}

fn main() {
    let result = do_things();
    if let Err(e) = result {
        panic!("{}", e);
    }
}