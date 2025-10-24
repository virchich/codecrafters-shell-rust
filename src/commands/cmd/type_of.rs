use crate::commands::command::Command;
use crate::commands::validator::{is_command_allowed, is_command_executable};
use std::env::var;

pub fn type_of(command: &Command) {
    if command.arguments.is_empty() || command.arguments.len() > 1 {
        eprintln!("type: must provide one argument");
        return;
    }

    let command_argument = command.arguments.first().unwrap();

    if is_command_allowed(command_argument) {
        println!("{} is a shell builtin", command_argument);
        return;
    }

    match var("PATH") {
        Ok(path) => {
            let (executable, executable_path) = is_command_executable(&command_argument, path);
            if executable {
                println!("{} is {}", command_argument, executable_path);
            } else {
                eprintln!("{}: not found", command_argument);
            }
            return;
        }
        Err(_) => {
            eprintln!("type: PATH variable not set");
            return;
        }
    }
}
