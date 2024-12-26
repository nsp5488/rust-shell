// Handles output of commands, including redirecting the output to files.
pub mod output {
    use std::fs::OpenOptions;
    use std::io::{BufWriter, Write};

    fn get_filewriter(file_path: &str, append: bool) -> Option<BufWriter<Box<dyn Write>>> {
        let file_result = OpenOptions::new()
            .append(append)
            .read(true)
            .write(true)
            .create(true)
            .open(file_path);
        if let Ok(file) = file_result {
            return Some(BufWriter::new(Box::new(file)));
        } else {
            return None;
        }
    }

    pub fn get_stdout(redirect: bool, file_path: &str, append: bool) -> BufWriter<Box<dyn Write>> {
        if redirect {
            if let Some(writer) = get_filewriter(file_path, append) {
                return writer;
            }
        }
        return BufWriter::new(Box::new(std::io::stdout().lock()));
    }

    pub fn get_stderr(redirect: bool, file_path: &str, append: bool) -> BufWriter<Box<dyn Write>> {
        if redirect {
            if let Some(writer) = get_filewriter(file_path, append) {
                return writer;
            }
        }
        return BufWriter::new(Box::new(std::io::stderr().lock()));
    }

    pub fn print(mut writer: impl std::io::Write, value: &[u8]) {
        let result = writer.write(value);

        match result {
            Ok(_) => (),
            Err(_) => eprint!("Could not write output"),
        }
        writer.flush().unwrap();
    }
}
