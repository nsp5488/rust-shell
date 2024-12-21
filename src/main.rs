use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write};

type Command = fn(&str) -> ();

fn build_commands() -> HashMap<String, Command> {
    let mut commands: HashMap<String, Command> = HashMap::new();

    commands.insert("exit".to_string(), handle_exit);
    commands.insert("echo".to_string(), handle_echo);
    commands.insert("type".to_string(), handle_type);
    return commands;
}
fn handle_exit(_command: &str) {
    std::process::exit(0);
}
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

fn handle_type(input: &str) {
    let known_commands = build_commands();
    let mut line = input.split(' ');

    line.next(); // discard 'type'
    let command = line.next();

    match known_commands.get(command.unwrap().trim()) {
        Some(_cmd) => print!("{} is a shell builtin\n", command.unwrap().trim()),
        None => print!("{}: not found\n", command.unwrap().trim()),
    }
}
fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    let commands: HashMap<String, fn(&str)> = build_commands();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();

        let mut line = input.split(' ');

        let func = commands.get(line.next().unwrap().trim());
        match func {
            Some(func) => func(&input),
            None => print_command_not_found(&input),
        }

        io::stdout().flush().unwrap();
    }
}
