use crate::commands::command::Command;
use crate::repl::history_store;
use std::io::Write;

pub fn history(command: &Command, output: &mut dyn Write) {
    let entries = history_store::get_all();
    let history_limit = if command.arguments.len() > 0 {
        match command.arguments[0].parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                output.write_all(format!("history: {}: numeric argument required", command.arguments[0]).as_bytes()).unwrap();
                return;
            }
        }
    } else {
        entries.len()
    };

    let history_entries_to_skip: usize = entries.len().saturating_sub(history_limit);

    for (i, command) in entries.iter().enumerate().skip(history_entries_to_skip) {
        writeln!(output, "  {} {}", i + 1, command).unwrap();
    }
}
