fn main() {
    let mut closure = lazy_add(41);
    let result = closure(1);
    println!("{}",result);
    println!("{}",lazy_add(41)(9));

    print_with_callback( || {
       "ok".to_string()
    });
}

fn lazy_add(i:i32) -> impl FnMut(i32) -> i32{
    let code = move |mut n:i32|{
        n=i+n;
        println!("i == {} n == {}",&i,&n);
        n
    };
    code
}

fn print_with_callback(cb:impl Fn()->String) -> () {
  let msg = cb();
  print!("{}",format!("*** {} *** ",msg));
}