enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

use self::Status::{Poor, Rich};
use self::Work::{Civilian,Soldier};

pub fn execute() {
    let mut status:Status = Poor;
    check_status(status);
    status = Rich;
    check_status(status);

    let mut work:Work = Soldier; 
    check_work(work);
    work = Civilian; 
    check_work(work);
}

fn check_work(w:Work){
   match w {
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}

fn check_status(s:Status){
    match s {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }
}
