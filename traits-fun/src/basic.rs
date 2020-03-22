#[derive(PartialEq, PartialOrd)]
pub struct Centimeters(f64);

#[derive(Debug)]
pub struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

pub fn fun_with_traits(){
    let foot = Inches(12);
    let meter = Centimeters(100.0);
    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };
    println!("One foot is {} than one meter.", cmp);
}