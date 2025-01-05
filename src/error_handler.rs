/// Print red colored text to stderr
///
/// # Arguments
///
/// * `text` - The text to print
fn print_red(text: &str) {
    eprintln!("\x1b[31m{}\x1b[0m", text);
}

/// Print yellow colored text to stderr
///
/// # Arguments
///
/// * `text` - The text to print
fn print_yellow(text: &str) {
    eprintln!("\x1b[33m{}\x1b[0m", text);
}

/// Print an error message and exit the program
///
/// # Arguments
///
/// * `error` - The error message to print
pub fn print_error(error: &str) {
    print_red(&format!("Error\n{}", error));
    std::process::exit(1);
}

// Commented out for prevent the warning message in terminal
/// Print a warning message
///
/// # Arguments
///
/// * `warning` - The warning message to print
pub fn print_warning(warning: &str) {
    print_yellow(&format!("Warning: {}", warning));
}