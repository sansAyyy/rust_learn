use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;

fn main() {
    match load_number("number.txt") {
        Ok(number) => println!("loaded number = {number}"),
        Err(error) => print_error_chain(&error),
    }

    match parse_number_from_text("abc") {
        Ok(number) => println!("parsed number = {number}"),
        Err(error) => print_error_chain(&error),
    }
}

#[derive(Debug)]
enum LoadNumberError {
    ReadFile { path: String, source: io::Error },
    ParseNumber { input: String, source: ParseIntError },
}

impl fmt::Display for LoadNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadNumberError::ReadFile { path, .. } => {
                write!(f, "failed to read number from '{path}'")
            }
            LoadNumberError::ParseNumber { input, .. } => {
                write!(f, "failed to parse '{input}' as a number")
            }
        }
    }
}

impl Error for LoadNumberError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LoadNumberError::ReadFile { source, .. } => Some(source),
            LoadNumberError::ParseNumber { source, .. } => Some(source),
        }
    }
}

fn load_number(path: &str) -> Result<i32, LoadNumberError> {
    let text = fs::read_to_string(path).map_err(|source| LoadNumberError::ReadFile {
        path: path.to_string(),
        source,
    })?;

    parse_number_from_text(&text)
}

fn parse_number_from_text(text: &str) -> Result<i32, LoadNumberError> {
    let trimmed = text.trim();
    trimmed
        .parse::<i32>()
        .map_err(|source| LoadNumberError::ParseNumber {
            input: trimmed.to_string(),
            source,
        })
}

fn print_error_chain(error: &dyn Error) {
    eprintln!("error: {error}");

    let mut source = error.source();
    while let Some(cause) = source {
        eprintln!("caused by: {cause}");
        source = cause.source();
    }
}
