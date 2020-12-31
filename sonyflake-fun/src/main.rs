use sonyflake::Sonyflake;

fn main() {
    let mut sf = Sonyflake::new().unwrap();
    let next_id = sf.next_id().unwrap();
    println!("Sonyflake id generator = {}", next_id);
}