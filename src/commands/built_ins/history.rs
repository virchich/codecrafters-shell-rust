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
                    let required_history = get_history(history_limit as i16);

                    for (i, command) in required_history.iter().enumerate() {
                        writeln!(output, "  {} {}", i + 1, command).unwrap();
                    }
                }
                Err(_) => {
                    writeln!(stderr, "history: {}: use either numeric value to show last N commands in history or \"-r\" option to load history from file", command.arguments[0]).unwrap();
                    return;
                }
            },
        }
    } else {
        let required_history = get_history(-1);

        for (i, command) in required_history.iter().enumerate() {
            writeln!(output, "  {} {}", i + 1, command).unwrap();
        }
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

fn get_history(n_entries_to_show: i16) -> Vec<String> {
    let entries = history_store::get_all();
    let history_entries_to_skip: usize = if n_entries_to_show < 0 {
        0
    } else {
        entries
            .len()
            .saturating_sub(n_entries_to_show.abs() as usize)
    };

    let mut output_history: Vec<String> = Vec::new();

    for command in entries.iter().skip(history_entries_to_skip) {
        output_history.push(command.to_string());
    }

    output_history
}
