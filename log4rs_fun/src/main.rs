use log::{info};
use log4rs;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    info!("booting up");
    info!("logging... ");
    println!("DONE");
}
