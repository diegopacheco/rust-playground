pub trait Greeter { fn greet(&self) -> String; }

struct Person { name: String }
impl Greeter for Person { fn greet(&self) -> String { format!("hello {}", self.name) } }

fn greet_all<T: Greeter>(items: &[T]) { for it in items { println!("{}", it.greet()); } }

pub fn run() {
    let a = Person { name: "ana".into() };
    let b = Person { name: "bob".into() };
    let v = vec![a,b];
    greet_all(&v);
}
