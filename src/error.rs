use crate::token::{Token, TokenType};

pub fn handle_error(message: &str) {
    eprintln!("{}", message);
    std::process::exit(65);
}

pub fn report(line: usize, message: &str) {
    let err = format!("[Line {line}] Error: {message}");
    eprintln!("{}", err);
}

