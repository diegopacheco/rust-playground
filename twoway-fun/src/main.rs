use std::str;
use std::ops::Index;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Str<'a>(pub &'a [u8]);

impl<'a> Str<'a> {
    fn len(&self) -> usize { self.0.len() }
    fn as_str(&self) -> &str { str::from_utf8(self.0).unwrap() }
}

impl<'a> Index<usize> for Str<'a> {
    type Output = u8;
    fn index(&self, ix: usize) -> &u8 {
        &self.0[ix - 1]
    }
}

fn main() {
    let p = Str(b"ababab").as_str();
    let x = Str(b"cbcbabababab").as_str();
    println!("Result is {:?}",twoway::find_str(&x, &p));
}
