use crate::token::{Token, TokenType};
use crate::{interpreter::RuntimeError, resolver::ResolveError};

pub fn handle_error(message: &str) {
    eprintln!("{}", message);
    std::process::exit(65);
}

pub fn report(line: usize, message: &str) {
    let err = format!("[Line {line}] Error: {message}");
    eprintln!("{}", err);
}

pub fn token_error(token: &Token, message: &str) {
    if token.get_type() == TokenType::Eof {
        report(token.get_line(), &("at end ".to_string() + message));
    } else {
        report(
            token.get_line(),
            &("at '".to_string() + token.get_lexeme() + "'. " + message),
        );
    }
}

#[derive(Debug)]
pub enum LoxError {
    ParseError(ParseError),
    ResolveError(ResolveError),
    RuntimeError(RuntimeError),
}

#[derive(Debug)]
pub struct ParseError(pub String);

impl From<ParseError> for LoxError {
    fn from(value: ParseError) -> Self {
        Self::ParseError(value)
    }
}

impl From<RuntimeError> for LoxError {
    fn from(value: RuntimeError) -> Self {
        Self::RuntimeError(value)
    }
}

impl From<ResolveError> for LoxError {
    fn from(value: ResolveError) -> Self {
        Self::ResolveError(value)
    }
}
