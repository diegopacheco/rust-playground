fn print_me<T> (value: &T) where T: std::fmt::Debug {
    println!("value is {:?}",value);
}

#[allow(dead_code)]
fn main(){

    #[derive(Debug)]
    struct IntPoint{
        x: i32,
        y: i32
    }

    struct CantDebug{}

    let i = 42;
    print_me(&i);
    print_me(&"hello".to_string());
    print_me(&true);
    print_me(&IntPoint{ x:1, y:2});

    // compilation error - error[E0277]: `CantDebug` doesn't implement `Debug`
    // print_me(&CantDebug{}); // `CantDebug` cannot be formatted using `{:?}`
}