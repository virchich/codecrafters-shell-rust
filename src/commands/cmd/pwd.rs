use crate::commands::command::Command;
use std::{env};

pub fn pwd(command: &Command) {
    match env::current_dir() {
        Ok(output) => {
            println!("{}", output.display());
        }
        Err(e) => {
            eprintln!("{}: {}", command.command, e);
        }
    }
}
