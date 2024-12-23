use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashMap, fs::DirEntry};
type Command = fn(&str) -> ();
use std::fs;
use std::path;

fn build_commands() -> HashMap<String, Command> {
    let mut commands: HashMap<String, Command> = HashMap::new();

    commands.insert("exit".to_string(), handle_exit);
    commands.insert("echo".to_string(), handle_echo);
    commands.insert("type".to_string(), handle_type);
    commands.insert("pwd".to_string(), handle_pwd);
    commands.insert("cd".to_string(), handle_cd);
    return commands;
}
fn handle_pwd(_command: &str) {
    print!("{}\n", std::env::current_dir().unwrap().display());
}

fn handle_cd(command: &str) {
    let mut line = command.split(' ');
    let cmd = line.next().unwrap().trim(); // discard "cd"
    let home = env::var("HOME").unwrap();
    let path_string = line.next().unwrap_or_else(|| "~").trim();

    let path = match path_string {
        "~" => path::Path::new(home.as_str()),
        _ => path::Path::new(path_string),
    };
    if let Ok(result) = path.try_exists() {
        if result {
            if let Err(e) = std::env::set_current_dir(path) {
                print!("{e}");
            }
        } else {
            print!("{cmd}: {}: No such file or directory\n", path.display())
        }
    } else {
        print!("{cmd}: {}: No such file or directory\n", path.display())
    }
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
    let mut dir_entry: Option<DirEntry> = None;

    if !found_path {
        return None;
    }
    let Ok(paths) = env::var("PATH") else {
        return None;
    };

    paths.split(':').for_each(|path| {
        if let Ok(mut directory) = fs::read_dir(path) {
            let found = directory.find(|element| {
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
        if let Some(func) = func {
            func(&input);
        } else {
            if let Some(entry) = search_in_path(command) {
                let args = line.map(|e| e.trim()).collect::<Vec<&str>>();
                let result = std::process::Command::new(entry.path()).args(args).output();
                if let Ok(value) = result {
                    let mut r = io::stdout().write(&value.stdout);

                    if r.is_err() {
                        print!("Error while writing results of {command}");
                    }
                    r = io::stderr().write(&value.stderr);
                    if r.is_err() {
                        print!("Error while writing errors of {command}");
                    }
                } else {
                    print!("Error while executing {command}");
                }
            } else {
                print_command_not_found(&input);
            }
        }

        io::stdout().flush().unwrap();
        io::stderr().flush().unwrap();
    }
}
