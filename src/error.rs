pub fn handle_error(message: &str) {
    eprintln!("{}", message);
    std::process::exit(65);
}

pub fn report(line: usize, message: &str) {
    let err = format!("[Line {line}] Error: {message}");
    eprintln!("{}", err);
}

pub enum LoxError {
    ParseError(ParseError),
    RuntimeError(RuntimeError),
}
pub struct RuntimeError(pub String);
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
