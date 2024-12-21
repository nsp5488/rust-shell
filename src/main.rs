#[allow(unused_imports)]
use std::io::{self, Write};

fn print_command_not_found(command: &str) {
    let result: String = format!("{command}: command not found");
    print!("{result}\n");
}

fn handle_echo(input: &str) {
    let mut line = input.split(' ');
    // discard the command
    line.next();

    // print each value
    let mut element = line.next();
    while element != None {
        print!("{}", element.unwrap());
        element = line.next();
        if element != None {
            print!(" ");
        }
    }
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

        let command = input.split(' ').next().unwrap().trim();

        match command {
            "exit" => return,
            "echo" => handle_echo(&input),
            _ => print_command_not_found(command),
        }

        io::stdout().flush().unwrap();
    }
}
