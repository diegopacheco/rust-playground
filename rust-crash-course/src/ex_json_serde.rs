use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User { id: u32, name: String, active: bool }

pub fn run() {
    print!(">>> Json serialization\n");

    let u = User { id: 1, name: "neo".into(), active: true };
    let s = serde_json::to_string(&u).unwrap();
    println!("{}", s);
    
    let d: User = serde_json::from_str(&s).unwrap();
    println!("{}", d.name);
}
