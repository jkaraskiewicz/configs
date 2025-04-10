use configs::execute;

fn main() {
    let output = execute().unwrap_or_else(|e| {
        let backtrace = e.backtrace();
        eprintln!("Error: {e}\n{}", backtrace);
        std::process::exit(1);
    });
    if !output.is_empty() {
        println!("{}", output);
    }
}
