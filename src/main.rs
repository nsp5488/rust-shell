mod shell_commands;
use crate::shell_commands::commands::{build_commands, execute_command_in_path};
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write};

fn read_eval_print(commands: HashMap<String, fn(&str)>) {
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        // read
        print!("$ ");
        io::stdout().flush().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();

        // evaluate
        let mut line = input.split(' ');
        let command = line.next().unwrap().trim();
        let func = commands.get(command);
        if let Some(func) = func {
            func(&input);
        } else {
            if let Some(value) = execute_command_in_path(command, line) {
                let (Ok(_res1), Ok(_res2)) = (
                    // print
                    io::stdout().write(&value.stdout),
                    io::stderr().write(&value.stderr),
                ) else {
                    panic!("Could not write to stdio or stderr");
                };
            } else {
                print!("{}: command not found", command.trim());
            }
        }

        // loop
        io::stdout().flush().unwrap();
        io::stderr().flush().unwrap();
    }
}

fn main() {
    let commands: HashMap<String, fn(&str)> = build_commands();
    read_eval_print(commands);
}
