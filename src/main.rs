use std::env; // for taking in args (only works on unicode)
use std::process; // for error handling
use minigrep::Config;


fn main() {
    // taking in the arguments
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Issue parsing arguments: {}", err);
        process::exit(1);
    });
}
