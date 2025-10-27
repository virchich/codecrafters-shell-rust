use crate::commands::cmd::cd::cd;
use crate::commands::cmd::echo::echo;
use crate::commands::cmd::exec::exec;
use crate::commands::cmd::exit::exit;
use crate::commands::cmd::pwd::pwd;
use crate::commands::cmd::type_of::type_of;
use crate::commands::command::Command;
use std::io;
use std::io::Write;

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
        "exit" => exit(command),
        "echo" => echo(command),
        "type" => type_of(command),
        "pwd" => pwd(command),
        "cd" => cd(command),
        _ => exec(command),
    }
}
