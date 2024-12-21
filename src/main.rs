use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashMap, fs::DirEntry};
type Command = fn(&str) -> ();
use std::fs;

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

fn search_in_path(command: &str) -> Option<DirEntry> {
    let found_path = env::var("PATH").is_ok();
    if !found_path {
        return None;
    }
    let paths = env::var("PATH");
    let mut dir_entry: Option<DirEntry> = None;
    paths.ok().unwrap().split(':').for_each(|path| {
        let directory = fs::read_dir(path);
        if directory.is_ok() {
            let mut listing = directory.ok().unwrap();
            let found = listing.find(|element| {
                element.is_ok()
                    && element.as_ref().ok().unwrap().file_name().to_str().unwrap()
                        == command.trim()
            });
            match found {
                Some(e) => dir_entry = e.ok(),
                None => (),
            }
        }
    });
    dir_entry
}

fn handle_type(input: &str) {
    let known_commands = build_commands();
    let mut line = input.split(' ');

    line.next(); // discard 'type'
    let command = line.next();

    match known_commands.get(command.unwrap().trim()) {
        Some(_cmd) => print!("{} is a shell builtin\n", command.unwrap().trim()),
        None => match search_in_path(command.unwrap()) {
            Some(entry) => print!(
                "{} is {}\n",
                command.unwrap().trim(),
                entry.path().canonicalize().ok().unwrap().display()
            ),
            None => print!("{}: not found\n", command.unwrap().trim()),
        },
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
        let command = line.next().unwrap().trim();
        let func = commands.get(command);
        match func {
            Some(func) => func(&input),
            None => match search_in_path(command) {
                Some(entry) => {
                    let args = line.map(|e| e.trim()).collect::<Vec<&str>>();
                    let result = std::process::Command::new(entry.path()).args(args).output();
                    match result {
                        Ok(value) => {
                            let r = io::stdout().write(&value.stdout);
                            if r.is_err() {
                                print!("Error while writing results of {command}");
                            }
                            ()
                        }
                        Err(e) => print!("Error while executing {command}: {e}"),
                    }
                }
                None => print_command_not_found(&input),
            },
        }

        io::stdout().flush().unwrap();
    }
}
