use std::env::{split_paths, var};
use std::path::{Path};
use crate::commands::command::Command;
use crate::commands::validator::is_command_allowed;

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
            type_path_handler(&command_argument, path);
            return;
        }
        Err(_) => {
            eprintln!("type: PATH variable not set");
            return;
        }
    }
}

fn type_path_handler(command: &String, paths: String) {
    for path in split_paths(&paths) {
        let dir = Path::new(path.as_path());

        if dir.exists() && dir.is_dir() {
            for dir_entry in std::fs::read_dir(dir).unwrap() {
                let entry = dir_entry.unwrap().path();

                if entry.ends_with(command) {
                    println!("{} is {}", command, entry.display());
                    return
                }
            }
        }
    }

    eprintln!("{}: not found", command);
}
