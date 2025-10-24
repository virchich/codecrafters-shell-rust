mod commands;

use crate::commands::validator::is_command_allowed;
use commands::core::{read_command, run_command};

fn repl() {
    loop {
        let command = read_command();

        if !is_command_allowed(&command) {
            eprintln!("{}: command not found", command.command);
            continue;
        }

        run_command(&command);
    }
}

fn main() {
    repl()
}
