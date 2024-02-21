fn main() {
    moved_values_borrowing_to_fix();

    let str = String::from("Hello Rust?");
    print_string(str.clone());
    // compile error
    //print_string(str); // error[E0382]: use of moved value: `str`
    print_string(str.clone());

    move_me();
}

fn moved_values_borrowing_to_fix(){
    let a = String::from("Hello World");
    let b = &a;
    
    // dont compile
    println!("{:?}",a);  // error[E0382]: borrow of moved value: `a`
    println!("{:?}",b);
}

fn print_string(s:String){
    println!("{:?}",s);
}

fn move_me(){
   let data = vec![1, 2, 3];
   let closure = move || println!("captured {data:?} by value");

   // compile error
   //println!("{:?}",data); // error[E0382]: borrow of moved value: `data`
   closure();
}