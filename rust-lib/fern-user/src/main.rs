extern crate fern_lib;

use fern_lib::Fern;
use fern_lib::run_simulation;

fn main() {
    let mut fern = Fern {
        size: 2.0,
        growth_rate: 3.0,
    };
    run_simulation(&mut fern,20);
    println!("Fern final size {}",&fern.size);
}