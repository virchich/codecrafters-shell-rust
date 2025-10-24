use std::io;
use std::io::Write;
use crate::commands::command::{echo, exit, Command};

pub fn read_command() -> Command {
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

pub fn run_command(command: &Command) {
    match command.command.as_str() {
        "exit" => {
            exit(command)
        }
        "echo" => {
            echo(command);
        }
        _ => {
            std::process::exit(255);
        }
    }
}
