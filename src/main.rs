use std::env; // for taking in args (only works on unicode)
use std::fs; // for manipulating files
use std::process; // for error handling

fn main() {
    // taking in the arguments
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Issue parsing arguments: {}", err);
        process::exit(1);
    });

    let content = fs::read_to_string(config.filename)
        .expect("Couldn't read the file properly");
}

// I like to keep my class functions without whitespace
struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
