use std::env; // for taking in args (only works on unicode)
use std::fs; // for manipulating files

fn main() {
    // taking in the arguments
    let args: Vec<String> = env::args().collect();

    // saving the arguments to variables
    let query = &args[1];
    let filename = &args[2];
    // [0] would be the program itself (minigrep)

    let content = fs::read_to_string(filename)
        .expect("Couldn't read the file properly");
}
