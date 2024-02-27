trait Quack {
    fn quack(&self);
}

struct CrazyDuck();

impl Quack for CrazyDuck {
    fn quack(&self) {
        println!("quack ack dak quack!? ");
    }
}

impl Quack for i32 {
    fn quack(&self) {
        for i in 1..*self+1 {
            print!("quack {} ",i);
        }
        println!("");
    }
}

fn main() {
    let d1 = CrazyDuck();
    let int = 3;
    let ducks: Vec<&dyn Quack> = vec![&d1,&CrazyDuck(),&CrazyDuck(),&int];
    for d in &ducks {
        d.quack();
    }
}
