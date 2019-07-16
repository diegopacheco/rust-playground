pub fn execute(){

    let x = 3.0;
    let y = 2.0;

    let result:Option<f64> =
        if y!=0.0 { Some(x/y) } else { None };

    println!("{}", match result {
        Some(f)  => format!("Horay! f is {}",f),
        None     => format!("{}","Oops!")
    });
}