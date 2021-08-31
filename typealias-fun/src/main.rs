
type Name = String;
type Age  = u8;

#[derive(Debug)]
struct Person{
    name: Name,
    age:Age,
}

type PV1 = Person;

fn main() {
 
    let john:PV1 = PV1 {
        name: String::from("John"),
        age: 44,
    };

    println!("{:?}",john);

}

