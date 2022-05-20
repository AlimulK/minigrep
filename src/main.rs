use std::env; // for taking in args (only works on unicode)
use std::process; // for error handling

use minigrep::Config;


fn main() {
    // taking in the arguments
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
