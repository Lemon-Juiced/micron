mod error_handler;

use error_handler::print_error;
use error_handler::print_warning;
use std::env;
use std::fs;

/// The main function of the Micron interpreter.
fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        // This should be theoretically when called from Micromake, but this can be called without Micromake outside of the development environment
        print_error("Usage: micron <filename>\nUsage: micron -r <filename>");
        print_warning(""); // Temporary fix for to prevent the warning message in terminal
    }

    // Get the filename
    let filename = &args[1];

    // Get the file extension, if file extension is not .mcrn, print an error message and exit the program
    let file_extension = filename.split('.').last().unwrap();
    if file_extension != "mcrn" {
        print_error("File extension must be .mcrn");
    }

    // Test: Print file contents
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("File contents:\n{}", contents);
}
