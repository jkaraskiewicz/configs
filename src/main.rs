#![warn(clippy::pedantic)]

use configs::execute;

fn main() {
    let output = execute().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    if !output.is_empty() {
        println!("{}", output);
    }
}
