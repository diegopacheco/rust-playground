use std::ops::Add;

#[derive(Debug)]
struct Account{
    id: u64,
    owner: String,
    value: f64,
}

impl Account{
    fn new(id:u64,owner:String,value:f64) -> Self{
        Account{
            id,
            owner,
            value,
        }
    }
}

impl Add for Account{
    type Output = Self;
    fn add(self, other:Self) -> Self::Output {
        Account{
            id: self.id + other.id,
            owner: format!("JointAccount({},{})",self.owner,other.owner),
            value: self.value + other.value,
        }
    }
}

fn main() {
    let a = Account::new(1,"Gandalfy".to_string(),100.0);
    let b = Account::new(2,"Melina".to_string(),100.0);
    let joint = a + b;
    println!("{:?}",joint);
}
