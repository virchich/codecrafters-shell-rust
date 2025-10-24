use crate::commands::command::Command;
use crate::commands::validator::is_command_allowed;

pub fn type_of(command: &Command) {
    if command.arguments.is_empty() || command.arguments.len() > 1 {
        eprintln!("type: must provide one argument");
        return;
    }

    if !is_command_allowed(command.arguments.first().unwrap()) {
        eprintln!("{}: not found", command.arguments.first().unwrap());
        return;
    } else {
        println!("{} is a shell builtin", command.arguments.first().unwrap());
        return;
    }

    eprintln!("{}: not found", command.arguments.first().unwrap());
}
