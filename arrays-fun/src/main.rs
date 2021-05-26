extern crate rand;
use crate::rand::Rng;

fn main() {
    let a: [u8; 3] = [1, 2, 3];
    println!("a: {:?}",a);

    let b: [&str; 3] = ["1", "2", "3"];
    println!("b: {:?}",b);

    let c: [String; 3] = [
        String::from("1"),
        String::from("2"),
        String::from("3")
    ];
    println!("c: {:?}",c);

    let mut rng = rand::thread_rng();
    let d: [u8; 3] = [rng.gen(), rng.gen(), rng.gen()];
    println!("d: {:?}",d);

    let e: [u8; 3] = [0; 3];
    println!("e: {:?}",e);

    let f: [Option<u8>; 3] = [None; 3];
    println!("f: {:?}",f);

    let g: [u8; 10] = [0; 10];
    println!("g: {:?}",g);

    let h: [u8; 3] = [rng.gen(); 3];
    println!("h: {:?}",h);

    let x: [Option<String>; 3] = Default::default();
    assert_eq!(
        [None, None, None],
        x
    );
}
