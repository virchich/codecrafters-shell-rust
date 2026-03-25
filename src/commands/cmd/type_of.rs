use crate::commands::command::Command;
use crate::commands::validator::{is_command_allowed, is_command_executable};
use std::env::var;
use std::io::Write;

pub fn type_of(command: &Command, writer_out: &mut dyn Write, writer_err: &mut dyn Write) {
    if command.arguments.is_empty() || command.arguments.len() > 1 {
        writeln!(writer_err, "type: must provide one argument").unwrap();
        return;
    }

    let command_argument = command.arguments.first().unwrap();

    if is_command_allowed(command_argument) {
        writeln!(writer_out, "{} is a shell builtin", command_argument).unwrap();
        return;
    }

    match var("PATH") {
        Ok(path) => {
            let (executable, executable_path) = is_command_executable(&command_argument, path);
            if executable {
                writeln!(writer_out, "{} is {}", command_argument, executable_path).unwrap();
            } else {
                writeln!(writer_err, "{}: not found", command_argument).unwrap();
            }
            return;
        }
        Err(_) => {
            writeln!(writer_err, "type: PATH variable not set").unwrap();
            return;
        }
    }
}
