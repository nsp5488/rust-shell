#[allow(unused_imports)]
use std::io::{self, Write};

fn print_command_not_found(command: &str) {
    let result: String = format!("{command}: command not found");
    print!("{result}\n");
}

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let cleaned = input.trim();
        print_command_not_found(cleaned);
        io::stdout().flush().unwrap();
    }
}
