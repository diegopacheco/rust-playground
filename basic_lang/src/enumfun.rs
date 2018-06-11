enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

pub fn execute() {
    use self::Status::{Poor, Rich};
    use self::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}
