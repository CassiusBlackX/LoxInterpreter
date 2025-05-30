mod callable;
mod environment;
mod error;
mod expr;
mod interpreter;
mod object;
mod parser;
mod resolver;
mod scanner;
mod stmt;
mod token;

use std::io::{self, Read, Write};

pub use error::LoxError;
use interpreter::Interpreter;
pub use interpreter::RuntimeError;
use parser::Parser;
use resolver::Resolver;
use scanner::Scanner;

fn run(source: String) -> Result<(), LoxError> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let mut interpreter = Interpreter::new();
    let mut resolver = Resolver::new(&mut interpreter);
    resolver.resolve_all(&statements)?;
    interpreter.interpret(&statements)?;

    Ok(())
}

pub fn run_file(path: String) -> Result<(), LoxError> {
    let mut file = std::fs::File::open(path).expect("failed to open file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read the file into String");
    run(contents)?;
    Ok(())
}

pub fn run_prompt() -> Result<(), LoxError> {
    println!("no file passed in, please input below");
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let bytes_read = stdin
            .read_line(&mut line)
            .expect("failed to read in stdin into String");

        if bytes_read == 0 {
            println!("user input end");
            break;
        }
        run(line)?;
    }
    Ok(())
}
