#[derive(Debug)]
struct Animal{
    name:String,
    color:String,
    friendly:bool,
}

fn main() {
    let garfield = Animal{
        name: String::from("Garfield"),
        color: String::from("Orange"),
        friendly: true,
    };
    println!("{:?}", garfield);

    let stray = Animal{
        name: String::from("Stray cat"),
        ..garfield
    };
    println!("{:?}", stray);
}
