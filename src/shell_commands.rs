pub mod commands {
    use std::{
        collections::HashMap,
        env,
        fs::{self, DirEntry},
        io::{Error, ErrorKind},
        path,
        process::Output,
    };
    type CommandResult = Result<(), Error>;
    pub type Command = fn(Vec<String>) -> CommandResult;

    pub fn build_commands() -> HashMap<String, Command> {
        let mut commands: HashMap<String, Command> = HashMap::new();

        commands.insert("exit".to_string(), handle_exit);
        commands.insert("echo".to_string(), handle_echo);
        commands.insert("type".to_string(), handle_type);
        commands.insert("pwd".to_string(), handle_pwd);
        commands.insert("cd".to_string(), handle_cd);
        return commands;
    }

    pub fn execute_command_in_path(command: &str, args: Vec<String>) -> Option<Output> {
        let mut command_output: Option<Output> = None;

        if let Some(entry) = search_in_path(command) {
            let result = std::process::Command::new(entry.path()).args(args).output();
            if let Ok(value) = result {
                command_output = Some(value);
            } else {
                print!("Error while executing {command}");
            }
        }
        return command_output;
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

    fn handle_pwd(_args: Vec<String>) -> CommandResult {
        print!("{}\n", std::env::current_dir().unwrap().display());
        Ok(())
    }

    fn handle_cd(args: Vec<String>) -> CommandResult {
        let home = env::var("HOME").unwrap();
        let path_string = match args.get(0) {
            Some(e) => e.as_str(),
            None => "~",
        };

        let path = match path_string {
            "~" => path::Path::new(home.as_str()),
            _ => path::Path::new(path_string),
        };
        let file_not_found = Error::new(
            ErrorKind::NotFound,
            format!("cd: {}: No such file or directory", path.display()),
        );
        if let Ok(result) = path.try_exists() {
            if result {
                if let Err(_e) = std::env::set_current_dir(path) {
                    return Err(file_not_found);
                }
            } else {
                return Err(file_not_found);
            }
        } else {
            return Err(file_not_found);
        }
        return Ok(());
    }

    fn handle_exit(_args: Vec<String>) -> CommandResult {
        std::process::exit(0);
    }

    fn handle_echo(args: Vec<String>) -> CommandResult {
        print!("{}\n", args.join(" "));
        Ok(())
    }

    fn handle_type(args: Vec<String>) -> CommandResult {
        let known_commands = build_commands();

        let command = match args.get(0) {
            Some(e) => e.as_str(),
            None => "",
        };

        match known_commands.get(command) {
            Some(_cmd) => print!("{} is a shell builtin\n", command),
            None => match search_in_path(command) {
                Some(entry) => print!(
                    "{} is {}\n",
                    command,
                    entry.path().canonicalize().ok().unwrap().display()
                ),
                None => {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("{}: not found", command),
                    ))
                }
            },
        }
        return Ok(());
    }
}
