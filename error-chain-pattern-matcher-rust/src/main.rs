#[macro_use]
extern crate error_chain;

error_chain!{
    foreign_links {
        Io(::std::io::Error);
    }
    errors {
        NoArgument(t: String) {
            display("no argument provided: '{}'", t)
        }
    }
}

fn run() -> Result<()> {
    use std::env::args;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    let file = args().skip(1).next()
        .ok_or(Error::from("my error msg: provide a file"))?;

    let f = File::open(&file)?;
    let mut l = 0;
    for line in BufReader::new(f).lines() {
        let line = line?;
        println!("{}", line);
        l += 1;
        if l == 10 {
            break;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        match e.kind() {
            &ErrorKind::Msg(ref s) => println!("msg {}",s),
            &ErrorKind::Io(ref s) => println!("io {}",s),
            _ => println!("??? Error {}", e.kind())
        }
        std::process::exit(1);
    }
}