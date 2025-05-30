use std::env;

use rustlox::{LoxError, RuntimeError, run_file, run_prompt};

fn main() -> Result<(), LoxError> {
    let mut args = env::args();
    // skip program name
    let _program_name = args.next();

    match args.next() {
        None => run_prompt(),
        Some(file_name) => {
            if args.next().is_some() {
                eprintln!("Usage: rustlox [script]");
                return Err(LoxError::RuntimeError(RuntimeError(
                    "incorrect cmd arguments".to_string(),
                )));
            }
            run_file(file_name)
        }
    }
}
