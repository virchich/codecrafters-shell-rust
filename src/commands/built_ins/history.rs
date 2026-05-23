use crate::commands::command::Command;
use crate::state::history_store;
use std::fs::OpenOptions;
use std::io::Write;

pub fn history(command: &Command, output: &mut dyn Write, stderr: &mut dyn Write) {
    if command.arguments.len() > 0 {
        match command.arguments[0].as_str() {
            "-a" => {
                if command.arguments.len() < 2 {
                    writeln!(stderr, "history: -a: filename argument required").unwrap();
                    return;
                }
                append_history_to_file(&command.arguments[1], stderr);
            }
            "-r" => {
                if command.arguments.len() < 2 {
                    writeln!(stderr, "history: -r: filename argument required").unwrap();
                    return;
                }
                load_history_from_file(&command.arguments[1], stderr);
            }
            "-w" => {
                if command.arguments.len() < 2 {
                    writeln!(stderr, "history: -w: filename argument required").unwrap();
                    return;
                }
                write_history_to_file(&command.arguments[1], stderr);
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

pub fn load_history_from_file(path: &str, stderr: &mut dyn Write) {
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

pub fn write_history_to_file(path: &str, stderr: &mut dyn Write) {
    let entries = history_store::get_all();

    match std::fs::write(path, entries.join("\n") + "\n") {
        Ok(_) => {}
        Err(e) => {
            writeln!(stderr, "history: {}: {}", path, e).unwrap();
        }
    }
}

fn append_history_to_file(path: &str, stderr: &mut dyn Write) {
    let current_history = history_store::get_all();

    match std::fs::exists(path) {
        Ok(_) => match std::fs::read_to_string(path) {
            Ok(_) => {
                let mut history_to_append: Vec<String> = vec![];

                for (i, command) in current_history.iter().rev().enumerate() {
                    if command.starts_with("history -a") && i > 0 {
                        break;
                    }

                    history_to_append.push(command.to_string());
                }

                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(path)
                    .unwrap();

                for command in history_to_append.iter().rev() {
                    writeln!(file, "{}", command).unwrap();
                }
            }
            Err(e) => {
                writeln!(stderr, "history: {}: {}", path, e).unwrap();
            }
        },
        Err(_) => {
            write_history_to_file(path, stderr);
        }
    }
}
