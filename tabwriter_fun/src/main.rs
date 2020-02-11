extern crate tabwriter;

use std::io::Write;
use tabwriter::TabWriter;

fn main() {
    let mut tw = TabWriter::new(vec![]);
    tw.write_all(b"
    Bruce Springsteen\tBorn to Run
    Bob Seger\tNight Moves
    Metallica\tBlack
    The Boss\tDarkness on the Edge of Town
    ").unwrap();
    tw.flush().unwrap();

    let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
    assert_eq!(&written, "
    Bruce Springsteen  Born to Run
    Bob Seger          Night Moves
    Metallica          Black
    The Boss           Darkness on the Edge of Town
    ");
    println!("{}",&written);
}
