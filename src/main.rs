#[allow(unused_imports)]
use std::io::{self, Write};

fn print_command_not_found(command: &str) {
    let result: String = format!("{}: command not found", command.trim());
    print!("{result}\n");
}

fn handle_echo(input: &str) {
    let mut line = input.split(' ');
    // discard the command
    line.next();

    print!("{}", line.collect::<Vec<&str>>().join(" "));
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

        let mut line = input.split(' ');

        match line.next().unwrap().trim() {
            "exit" => return,
            "echo" => handle_echo(&input),
            _ => print_command_not_found(&input),
        }

        io::stdout().flush().unwrap();
    }
}
