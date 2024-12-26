mod parse;
mod shell_commands;

use crate::parse::parser::parse_input;
use crate::shell_commands::commands::{build_commands, execute_command_in_path};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::BufWriter;
#[allow(unused_imports)]
use std::io::{self, Write};

fn get_filewriter(file_path: &str, append: bool) -> Option<BufWriter<Box<dyn Write>>> {
    let file_result = OpenOptions::new().append(append).open(file_path);
    if let Ok(file) = file_result {
        return Some(BufWriter::new(Box::new(file)));
    } else {
        return None;
    }
}

fn get_stdout(redirect: bool, file_path: &str, append: bool) -> BufWriter<Box<dyn Write>> {
    if redirect {
        if let Some(writer) = get_filewriter(file_path, append) {
            return writer;
        }
    }
    return BufWriter::new(Box::new(std::io::stdout().lock()));
}

fn get_stderr(redirect: bool, file_path: &str, append: bool) -> BufWriter<Box<dyn Write>> {
    if redirect {
        if let Some(writer) = get_filewriter(file_path, append) {
            return writer;
        }
    }
    return BufWriter::new(Box::new(std::io::stderr().lock()));
}

fn print(mut writer: impl std::io::Write, value: &[u8]) {
    let result = writer.write(value);

    match result {
        Ok(_) => (),
        Err(_) => eprint!("Could not write output"),
    }
    writer.flush().unwrap();
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

        let parsed_data = match parsed {
            Some(e) => e,
            None => {
                print(std::io::stderr(), "Error parsing input\n".as_bytes());
                continue;
            }
        };
        let out_writer = get_stdout(
            parsed_data.redirect_stdout,
            &parsed_data.stdout_path,
            parsed_data.append_stdout,
        );
        let err_writer = get_stderr(
            parsed_data.redirect_stderr,
            &parsed_data.stderr_path,
            parsed_data.append_stderr,
        );
        // evaluate
        let func = commands.get(parsed_data.command.as_str());
        if let Some(func) = func {
            match func(parsed_data.args) {
                Ok(value) => print(out_writer, &value.as_bytes()),
                Err(e) => print(err_writer, &e.to_string().as_bytes()),
            };
        } else {
            match execute_command_in_path(parsed_data.command.as_str(), parsed_data.args) {
                Ok(output) => {
                    if let Some(value) = output {
                        print(out_writer, &value.stdout);
                        print(err_writer, &value.stderr);
                    }
                }
                Err(e) => {
                    print(err_writer, e.to_string().as_bytes());
                }
            }
        }

        // loop
    }
}

fn main() {
    let commands: HashMap<String, crate::shell_commands::commands::Command> = build_commands();
    read_eval_print(commands);
}
