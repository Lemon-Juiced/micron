use std::env;
use std::fs;

// The Micron Interpreter Itself
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Interpreter Usage: micron <file>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("File contents:\n{}", contents);
}
