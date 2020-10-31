extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let mut sh = Md5::new();
    sh.input_str("this is a string");
    println!("this is a string (md5) == {}", sh.result_str());
}
