fn len(s: &String) -> usize { s.len() }

pub fn run() {
    let mut s = String::from("abc");
    let l = len(&s);
    println!("{}", l);
    let r1 = &s;
    let r2 = &s;
    println!("{}{}", r1, r2);
    let r3 = &mut s;
    r3.push('d');
    println!("{}", r3);
}
