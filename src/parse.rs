pub mod parser {
    use std::str::CharIndices;
    #[derive(Debug)]
    pub struct ParsedData {
        pub command: String,
        pub args: Vec<String>,
        pub redirect_info: RedirectInfo,
    }
    #[derive(Debug)]
    pub struct RedirectInfo {
        pub redirect_stdout: bool,
        pub append_stdout: bool,
        pub stdout_path: String,
        pub redirect_stderr: bool,
        pub append_stderr: bool,
        pub stderr_path: String,
        remaining: String,
    }

    impl ParsedData {
        fn new(command: String, args: Vec<String>, redirect_info: RedirectInfo) -> ParsedData {
            return ParsedData {
                command,
                args,
                redirect_info,
            };
        }
    }
    impl RedirectInfo {
        fn new(
            stdout_path: String,
            stderr_path: String,
            append_stdout: bool,
            append_stderr: bool,
            remaining: String,
        ) -> RedirectInfo {
            let redirect_stdout = !stdout_path.is_empty();
            let redirect_stderr = !stderr_path.is_empty();
            return RedirectInfo {
                redirect_stdout,
                append_stdout,
                stdout_path,
                redirect_stderr,
                append_stderr,
                stderr_path,
                remaining,
            };
        }
    }

    fn extract_filename(chars: &mut CharIndices<'_>) -> String {
        let mut filename: Vec<char> = Vec::new();
        while let Some(c) = chars.next() {
            if c.1.is_whitespace() {
                return String::from_iter(filename.iter());
            } else {
                filename.push(c.1);
            }
        }
        return "".to_string();
    }

    fn parse_redirect(input: &String) -> RedirectInfo {
        let mut chars = input.char_indices();
        let mut previous: char = 'a'; // default value that is not a special case.
        let mut stdout_path: String = "".to_string();
        let mut stderr_path: String = "".to_string();

        let mut pre_redirect: Vec<char> = Vec::new();
        let mut found_redirect: bool = false;
        let mut append_stdout = false;
        let mut append_stderr = false;
        while let Some(c) = chars.next() {
            if c.1 == '>' && previous != '\\' {
                found_redirect = true;
                let nextchar = chars.next();

                if previous == '1' || previous == ' ' {
                    // redirect stdout
                    append_stdout = nextchar.unwrap_or((1, 'a')).1 == '>';
                    if append_stdout {
                        chars.next();
                    }
                    stdout_path = extract_filename(&mut chars);
                }
                if previous == '2' {
                    // redirect stderr
                    append_stderr = nextchar.unwrap_or((1, 'a')).1 == '>';
                    if append_stderr {
                        chars.next();
                    }
                    stderr_path = extract_filename(&mut chars);
                }
            } else {
                if !found_redirect {
                    pre_redirect.push(previous);
                }
                previous = c.1;
            }
        }

        // Rebuild the original string upto the beginning of the redirects
        pre_redirect.push('\n');
        let mut iter_pre_redirect = pre_redirect.iter();
        iter_pre_redirect.next(); // consume the 'a' default value
        let remaining = String::from_iter(iter_pre_redirect);

        return RedirectInfo::new(
            stdout_path,
            stderr_path,
            append_stdout,
            append_stderr,
            remaining,
        );
    }

    pub fn parse_input(input: &String) -> Option<ParsedData> {
        let mut parsed_input: Vec<String> = Vec::new();

        let mut found_match = false;

        let redirect_info = parse_redirect(input);
        let mut chars = redirect_info.remaining.char_indices();
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
                    } else if inner.1 == '\\' {
                        if let Some(next) = chars.next() {
                            match next.1 {
                                '\\' => current_word.push('\\'),
                                '$' => current_word.push('$'),
                                '"' => current_word.push('"'),
                                _ => {
                                    current_word.push('\\');
                                    current_word.push(next.1);
                                }
                            }
                        }
                    } else {
                        current_word.push(inner.1);
                    }
                }
                if !found_match {
                    return None;
                }
                found_match = false;
            } else if c.1 == '\\' {
                if let Some(escaped) = chars.next() {
                    current_word.push(escaped.1);
                }
            } else {
                current_word.push(c.1);
            }
        }
        let command: String;
        let args: Vec<String>;
        let mut iter = parsed_input.into_iter();
        command = iter.next().unwrap_or_else(|| "".to_string());
        args = iter.collect();

        let parsed_data = ParsedData::new(command, args, redirect_info);
        // handle spaces
        return Some(parsed_data);
    }
}
