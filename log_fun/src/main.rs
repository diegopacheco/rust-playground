#[macro_use] extern crate log;
extern crate simplelog;

use log::{info, trace, warn};
use simplelog::*;
use std::fs::File;

fn main() {

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed).unwrap(),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    println!("Hello, world!");
    let _ = info!("Razor located: {}", "gilete");
    let _ = trace!("Commencing yak shaving");
    let _ = warn!("Unable to locate a razor: {}, retrying","ops");
    println!("DONE.");
}
