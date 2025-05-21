pub fn handle_error(message: String) {
    eprintln!("{}", message);
    std::process::exit(65);
}

pub fn report(line: usize, message: String) {
    let err = format!("[Line {line}] Error: {message}");
    handle_error(err);
}
