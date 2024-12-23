pub mod commands {
    use std::{
        collections::HashMap,
        env,
        fs::{self, DirEntry},
        path,
        process::Output,
    };
    type Command = fn(&str) -> ();

    pub fn build_commands() -> HashMap<String, Command> {
        let mut commands: HashMap<String, Command> = HashMap::new();

        commands.insert("exit".to_string(), handle_exit);
        commands.insert("echo".to_string(), handle_echo);
        commands.insert("type".to_string(), handle_type);
        commands.insert("pwd".to_string(), handle_pwd);
        commands.insert("cd".to_string(), handle_cd);
        return commands;
    }

    pub fn execute_command_in_path(
        command: &str,
        line: core::str::Split<'_, char>,
    ) -> Option<Output> {
        let mut command_output: Option<Output> = None;

        if let Some(entry) = search_in_path(command) {
            let args = line.map(|e| e.trim()).collect::<Vec<&str>>();
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
}
