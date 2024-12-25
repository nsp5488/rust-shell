mod shell_commands;
use crate::shell_commands::commands::{build_commands, execute_command_in_path};
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write};

fn parse_input(input: &String) -> Option<Vec<String>> {
    let mut parsed_input: Vec<String> = Vec::new();

    // handle quotes
    let mut chars = input.char_indices();
    let mut found_match = false;

    let mut current_word: Vec<char> = Vec::new();
    while let Some(c) = chars.next() {
        if c.1.is_ascii_whitespace() {
            // end arg if not in a quote
            if current_word.len() == 0 {
                continue;
            }
            let s = String::from_iter(current_word.iter());
            current_word.clear();
            parsed_input.push(s);
        } else if c.1 == '\'' {
            // consume characters until we find a matching quote
            while let Some(inner) = chars.next() {
                if inner.1 == '\'' {
                    found_match = true;
                    break;
                } else {
                    current_word.push(inner.1);
                }
            }
            if !found_match {
                return None;
            }
            found_match = false;
        } else if c.1 == '\"' {
            while let Some(inner) = chars.next() {
                if inner.1 == '\"' {
                    found_match = true;
                    break;
                } else {
                    current_word.push(inner.1);
                }
            }
            if !found_match {
                return None;
            }
            found_match = false;
        } else {
            current_word.push(c.1);
        }
    }

    // handle spaces
    return Some(parsed_input);
}

fn read_eval_print(commands: HashMap<String, crate::shell_commands::commands::Command>) {
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        // read
        print!("$ ");
        io::stdout().flush().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let parsed = parse_input(&input);
        let command: String;
        let args: Vec<String>;
        match parsed {
            Some(e) => {
                let mut iter = e.into_iter();
                command = iter.next().unwrap_or_else(|| "".to_string());
                args = iter.collect();
            }
            None => {
                io::stderr()
                    .write("Error parsing input\n".as_bytes())
                    .expect("Panic if we cannot write to screen");
                continue;
            }
        }
        // evaluate

        let func = commands.get(command.as_str());
        if let Some(func) = func {
            if let Err(e) = func(args) {
                print!("{}\n", e.to_string());
            };
        } else {
            if let Some(value) = execute_command_in_path(command.as_str(), args) {
                let (Ok(_res1), Ok(_res2)) = (
                    // print
                    io::stdout().write(&value.stdout),
                    io::stderr().write(&value.stderr),
                ) else {
                    panic!("Could not write to stdio or stderr");
                };
            } else {
                print!("{}: command not found\n", command.as_str());
            }
        }

        // loop
        io::stdout().flush().unwrap();
        io::stderr().flush().unwrap();
    }
}

fn main() {
    let commands: HashMap<String, crate::shell_commands::commands::Command> = build_commands();
    read_eval_print(commands);
}
