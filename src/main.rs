#[allow(unused_imports)]
use std::io::{self, Write};

fn is_command_allowed(command: &str) -> bool {
    let allowed_commands: [String; 0] = [];

    allowed_commands.contains(&command.to_string())
}

fn read_command() -> String {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();

    command
}

fn repl() {
    loop {
        let command = read_command();

        if !is_command_allowed(&command) {
            eprintln!("{}: command not found", command.trim());
        }
    }
}

fn main() {
    repl()
}
