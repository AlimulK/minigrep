use std::env;
// this only works for unicode

fn main() {
    // taking in the arguments
    let args: Vec<String> = env::args().collect();

    // saving the arguments to variables
    let query = &args[1];
    let filename = &args[2];
    // [0] would be the program itself (minigrep)

}
