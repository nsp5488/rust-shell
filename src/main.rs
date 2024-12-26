mod output;
mod parse;
mod shell_commands;

use crate::output::output::{get_stderr, get_stdout, print};
use crate::parse::parser::parse_input;
use crate::shell_commands::commands::{build_commands, execute_command_in_path};

use std::collections::HashMap;
use std::io::Write;

// Main REPL loop for the shell.
fn read_eval_print(commands: HashMap<String, crate::shell_commands::commands::Command>) {
    let stdin = std::io::stdin();
    let mut input = String::new();

    loop {
        // read
        print!("$ ");
        std::io::stdout().flush().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();

        // parse
        let parsed = parse_input(&input);
        let parsed_data = match parsed {
            Some(e) => e,
            None => {
                print(std::io::stderr(), "Error parsing input\n".as_bytes());
                continue;
            }
        };
        let out_writer = get_stdout(
            parsed_data.redirect_info.redirect_stdout,
            &parsed_data.redirect_info.stdout_path,
            parsed_data.redirect_info.append_stdout,
        );
        let err_writer = get_stderr(
            parsed_data.redirect_info.redirect_stderr,
            &parsed_data.redirect_info.stderr_path,
            parsed_data.redirect_info.append_stderr,
        );

        // evaluate
        let func = commands.get(parsed_data.command.as_str());
        if let Some(func) = func {
            // print
            match func(parsed_data.args) {
                Ok(value) => print(out_writer, &value.as_bytes()),
                Err(e) => print(err_writer, &e.to_string().as_bytes()),
            };
        } else {
            match execute_command_in_path(parsed_data.command.as_str(), parsed_data.args) {
                // print
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
    // Get shell builtins
    let commands: HashMap<String, crate::shell_commands::commands::Command> = build_commands();
    read_eval_print(commands);
}
