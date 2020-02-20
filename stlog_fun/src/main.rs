extern crate stlog;

use stlog::Log;
use stlog::{global_logger, GlobalLog};

#[cfg(feature = "spanned")]
use stlog::spanned::{error, info, trace};
#[cfg(not(feature = "spanned"))]
use stlog::{error, info};

struct Logger;
impl Log for Logger {
    type Error = ();
    fn log(&mut self, byte: u8) -> Result<(), ()> {
        println!("{}", byte);
        Ok(())
    }
}
impl GlobalLog for Logger {
    fn log(&self, _: u8) {}
}

#[global_logger]
static LOGGER: Logger = Logger;

fn main() {
    let mut logger = Logger;
    info!(logger, "Hello!").unwrap();
    error!("Bye!");
}
