pub struct Fern{
    pub size:f64,
    pub growth_rate:f64,
}

impl Fern{
    pub fn grow(&mut self){
        self.size *= 1.0 + self.growth_rate;
    }
}

pub fn run_simulation(fern:&mut Fern,days:usize){
    for _ in 0..days{
        fern.grow();
    }
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn fern_simulation_test() {
        use crate::Fern;
        use crate::run_simulation;

        let mut fern = Fern {
            size: 2.0,
            growth_rate: 3.0,
        };
        run_simulation(&mut fern,20);
        println!("Fern final size {}",&fern.size);
    }
}
