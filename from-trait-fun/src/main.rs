use std::convert::From;

#[derive(Debug)]
struct Person {
    id: u32,
    name:String
}

#[derive(Debug)]
struct Employee {
    id: u32,
    name:String
}

impl From<Person> for Employee {
    fn from(person:Person) -> Self {
        Employee{
            id: person.id,
            name: person.name
        }
    }
}

fn main() {
    let person = Person{
        id: 1,
        name: "John Doe".to_string()
    };
    let employ = Employee::from(person);
    println!("{:?}",employ);
}
