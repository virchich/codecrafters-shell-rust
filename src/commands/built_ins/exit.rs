use crate::commands::built_ins::history::write_history_to_file;
use crate::commands::command::Command;
use std::env;
use std::io::Write;

pub fn exit(command: &Command, output: &mut dyn Write) {
    match env::var("HISTFILE") {
        Ok(path) => {
            write_history_to_file(&path, output);
        }
        Err(_) => {}
    }

    if command.arguments.is_empty() {
        std::process::exit(0);
    }

    match command.arguments[0].parse::<i32>() {
        Ok(code) => std::process::exit(code),
        Err(_) => {
            output.write_all(format!("exit: {}: numeric argument required", command.arguments[0]).as_bytes()).unwrap();
            std::process::exit(255);
        }
    }
}
