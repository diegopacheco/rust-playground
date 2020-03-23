fn main() {
    let x = Some(42);
    if let Some(i) = x {
        println!("if_let working {}",i);
    }else{
        println!("oops!");
    }
}
