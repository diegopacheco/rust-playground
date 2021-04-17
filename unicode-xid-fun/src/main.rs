extern crate unicode_xid;
use unicode_xid::UnicodeXID;

fn main() {
    let ch = 'a';
    println!("Is [{}] a valid start of an identifier? {}", ch, UnicodeXID::is_xid_start(ch));

    let ch2 = '\r';
    println!("Is [{}] a valid start of an identifier? {}", ch2, UnicodeXID::is_xid_start(ch2));
}
