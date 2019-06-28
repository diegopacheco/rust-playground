extern crate clap; 
use clap::{Arg, App};
 
fn main() { 
    let matches = App::new("clapfun")
       .version("1.0.0")
       .about("Does great rusty things!")
       .author("Diego Pacheco.")
        .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file")
                               .takes_value(true))
       .get_matches(); 

    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);   
}