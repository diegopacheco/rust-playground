#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod person;
use person::Person;

use crate::person::Address;

fn main() {
    // Json to Struct
    let data = r#" {
        "name": "John Doe", "age": 43,
        "address": {"street": "main", "city":"Downtown"},
        "phones":["27726550023"]
       } "#;
    let p: Person = serde_json::from_str(data).expect("deserialize error");
    println!("Please call {} at the number {}", p.name, p.phones[0]);
    println!("{:#?}", p);

    // Struct to Json   
    let jd = Person{
        name: "John Doe".into(),
        age: 62,
        phones: vec!("12354334534".into()),
        address: Address{
            street: "123 popular st".into(),
            city: "Wonderland".into(),
        }
    };
    println!("{:#?}", serde_json::to_string(&jd));
}
