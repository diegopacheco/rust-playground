macro_rules! create_struct {
    ($name:ident { $($field:ident: $type:ty),* }) => {
        #[derive(Debug)]
        struct $name {
            $($field: $type),*
        }
        
        impl $name {
            fn new($($field: $type),*) -> Self {
                $name {
                    $($field),*
                }
            }
        }
    }
}

fn main() {
    create_struct!(Person {
        name: String,
        age: u32,
        height: f64
    });
    
    let person = Person::new(
        String::from("John Doe"),
        30,
        1.75
    );
    
    println!("{:#?}", person);
}