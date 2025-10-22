#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let allowed_commands: [String; 0] = [];

    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();

    if !allowed_commands.contains(&command) {
        eprintln!("{}: command not found", command.trim());
        exit(1);
    }
}
