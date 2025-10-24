#[allow(unused_imports)]
use std::io::{self, Write};

struct Command {
    command: String,
    arguments: Vec<String>,
}

fn read_command() -> Command {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();

    let parts: Vec<String> = command
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    Command {
        command: parts[0].clone(),
        arguments: parts[1..].to_vec(),
    }
}

fn is_command_allowed(command: &Command) -> bool {
    let allowed_commands = ["exit".to_string()];

    allowed_commands.contains(&command.command)
}

fn run_command(command: &Command) {
    match command.command.as_str() {
        "exit" => {
            exit(command)
        }
        _ => {}
    }
}

fn exit(command: &Command) {
    if command.arguments.is_empty() {
        std::process::exit(0);
    }

    match command.arguments[0].parse::<i32>() {
        Ok(code) => std::process::exit(code),
        Err(_) => {
            eprintln!("exit: {}: numeric argument required", command.arguments[0]);
            std::process::exit(255);
        }
    }
}

fn repl() {
    loop {
        let command = read_command();

        if !is_command_allowed(&command) {
            eprintln!("{}: command not found", command.command);
        }

        run_command(&command);
    }
}

fn main() {
    repl()
}
