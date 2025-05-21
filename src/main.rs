use std::env;

use rustlox::{run_file, run_prompt};

fn main() -> Result<(), String> {
    let mut args = env::args();
    // skip program name
    let _program_name = args.next();

    match args.next() {
        None => run_prompt().map_err(|_| "error when running run_prompt!".to_string())?,
        Some(file_name) => {
            if args.next().is_some() {
                eprintln!("Usage: rustlox [script]");
                return Err("incorrect cmd arguments".to_string());
            }
            run_file(file_name).map_err(|_| "error when running run_file!".to_string())?
        }
    }
    Ok(())
}
