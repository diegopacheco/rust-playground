use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

struct FlatFile<R> {
    reader: io::BufReader<R>,
    file_name: String,
    buf: String,
}

#[allow(dead_code)]
impl <R: Read> FlatFile<R> {
    fn new(r: R) -> FlatFile<R> {
        FlatFile{reader: io::BufReader::new(r), buf: String::new(), file_name: "???".into()}
    }

    fn from_str(str: &str) -> FlatFile<BufReader<File>> {
        let file_name: String = String::from(str);
        let file: File = File::open(&file_name).unwrap();
        let reader: BufReader<File> = io::BufReader::new(file);
        FlatFile{reader: io::BufReader::new(reader), buf: String::new(),file_name: file_name}
    }

    fn next<'a>(&'a mut self) -> Option<io::Result<&'a str>>{
        self.buf.clear();
        match self.reader.read_line(&mut self.buf) {
            Ok(nbytes) => if nbytes == 0 {
                None 
            } else {
                let line = self.buf.trim_end();
                Some(Ok(line))
            },
            Err(e) => Some(Err(e))
        }
    }    

    fn read_all_lines(filename: &str) -> io::Result<()> {
        let file = File::open(&filename)?;
        let mut lines = FlatFile::new(file);
        while let Some(line) = lines.next() {
            let line = line?;
            println!("{}", line);
        }
        Ok(())
    }
}

impl Display for FlatFile<BufReader<File>> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "FlatFile[{}]", self.file_name)
    }
}

fn main() {

    let mut data = FlatFile::<BufReader<File>>::from_str("data.txt");
    println!("Flatfile {}", data);

    let mut i = 1;
    while let Some(line) = data.next() {
        let line = line.unwrap();
        println!("[line {i}] = {}", line);
        i +=1;
    }

    println!("Let's read again!");
    FlatFile::<BufReader<File>>::read_all_lines("data.txt").unwrap();
   
}
