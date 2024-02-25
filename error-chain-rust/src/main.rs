#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
        }
    }
}
use errors::*;

fn run() -> Result<()> {
    use std::fs::File;
    File::open("file")?;
    Ok(())
}


fn main() {
    if let Err(e) = run() {
        println!("error: {}", e);
        std::process::exit(1);
    }
}