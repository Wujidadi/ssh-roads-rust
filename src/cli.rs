use std::io::{self, Write};

pub fn read_input() -> String {
    let mut input = String::new();
    print!("");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}
