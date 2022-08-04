use derive_new::new;

#[derive(Debug, Default, new)]
struct Person{
    #[new(default)] id: i32,
    #[new(default)] mail: String,
    name: String,
}

fn main() {
    let john = Person::new("John".to_owned());
    println!("Hello, world! {:?}",john);
}
