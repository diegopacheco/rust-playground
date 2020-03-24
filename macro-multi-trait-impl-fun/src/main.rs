use std::fmt;

#[derive(Debug)]
struct Dog{
    name: String,
}

#[derive(Debug)]
struct Cat{
    name: String,
}

trait Animal{
    fn greet(&self) -> String;
}

impl Animal for Dog{
    fn greet(&self) -> String{
        return String::from("Rof Rof rof");
    }
}

impl Animal for Cat{
    fn greet(&self) -> String {
        return String::from("meau meau meau");
    }
}

macro_rules! impl_T {
    (for $($t:ty),+) => {
        $(impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.greet())
            }
        })*
    }
}
impl_T!(for Dog,Cat);

fn main() {
    let cat = Cat{
        name: String::from("Melina"),
    };
    let dog = Dog{
        name: String::from("Max"),
    };
    println!("cat {:?} dot {:?}", cat,dog);
    println!("{} {} ", cat.greet(), dog.greet());
    
    let b = Box::from(cat);
    println!("{}", b);
    let c = Box::from(dog);
    println!("{}", c);
}
