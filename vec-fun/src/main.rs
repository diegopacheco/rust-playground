use std::fmt::{Debug};

fn main() {

    #[derive(Debug)]
    struct Person{
        age:i8,
        name:String,
        gender:String
    }

    let people:Vec<Person> = vec![Person{
        age:36,
        name: String::from("Diego"),
        gender: String::from("male")
    }, Person{
          age:37,
          name: String::from("Dessa"),
          gender: String::from("female")
      },
      Person{
          age:0,
          name: String::from("Clara"),
          gender: String::from("female")
      }
    ];

    for person in people{
        println!("{:?}",person);
    }
}
