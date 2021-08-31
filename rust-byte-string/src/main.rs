fn main() {
    let b_string = b"STR";
    if b_string == &[b'S',b'T',b'R']{
        println!("Same string {:?} {:?}",&b_string,&[b'S',b'T',b'R']);
    } else{
        println!("Different string");
    }
}
