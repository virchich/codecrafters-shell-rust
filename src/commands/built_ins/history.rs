use crate::commands::command::Command;
use crate::repl::history_store;
use std::io::Write;

pub fn history(command: &Command, output: &mut dyn Write, stderr: &mut dyn Write) {
    if command.arguments.len() > 0 {
        match command.arguments[0].as_str() {
            "-r" => {
                if command.arguments.len() < 2 {
                    writeln!(stderr, "history: -r: filename argument required").unwrap();
                    return;
                }
                load_history_from_file(&command.arguments[1], stderr);
            }
            other => match other.parse::<usize>() {
                Ok(history_limit) => {
                    print_history(history_limit as i16, output);
                }
                Err(_) => {
                    writeln!(stderr, "history: {}: use either numeric value to show last N commands in history or \"-r\" option to load history from file", command.arguments[0]).unwrap();
                    return;
                }
            },
        }
    } else {
        print_history(-1, output);
    }
}

fn print_history(n_entries_to_show: i16, output: &mut dyn Write) {
    let entries = history_store::get_all();
    let skip = if n_entries_to_show < 0 {
        0
    } else {
        entries.len().saturating_sub(n_entries_to_show as usize)
    };

    for (i, command) in entries.iter().enumerate().skip(skip) {
        writeln!(output, "{:>5}  {}", i + 1, command).unwrap();
    }
}

fn load_history_from_file(path: &str, stderr: &mut dyn Write) {
    match std::fs::read_to_string(path) {
        Ok(contents) => {
            for line in contents.lines() {
                if !line.is_empty() {
                    history_store::push(line.to_string());
                }
            }
        }
        Err(e) => {
            writeln!(stderr, "history: {}: {}", path, e).unwrap();
        }
    }
}
