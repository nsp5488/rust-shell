#[allow(unused_imports)]
use std::io::{self, Write};

fn print_command_not_found(command: &str) {
    let result: String = format!("{command}: command not found");
    print!("{result}\n");
}

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let cleaned = input.trim();

        match cleaned {
            "exit 0" => return,
            _ => print_command_not_found(cleaned),
        }

        io::stdout().flush().unwrap();
    }
}
