mod error;
mod scanner;
mod token;

use std::io::{self, Read, Write};

use scanner::Scanner;

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{token}");
    }
}

pub fn run_file(path: String) -> io::Result<()> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    run(contents);
    Ok(())
}

pub fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let bytes_read = stdin.read_line(&mut line)?;

        if bytes_read == 0 {
            println!("user input end");
            break;
        }
        run(line);
    }
    Ok(())
}
