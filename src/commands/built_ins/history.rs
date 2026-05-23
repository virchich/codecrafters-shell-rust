use crate::state::history_store;
use crate::syntax::command_invocation::CommandInvocation;
use std::fs::OpenOptions;
use std::io::Write;

pub fn history(command: &CommandInvocation, output: &mut dyn Write, stderr: &mut dyn Write) {
    if !command.arguments.is_empty() {
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

    match std::path::Path::new(path).try_exists() {
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

#[cfg(test)]
mod tests {
    use super::{history, load_history_from_file, write_history_to_file};
    use crate::state::history_store;
    use crate::syntax::command_invocation::CommandInvocation;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "rust-shell-{}-{}-{}",
            label,
            std::process::id(),
            unique
        ))
    }

    fn command(arguments: &[&str]) -> CommandInvocation {
        CommandInvocation {
            name: "history".to_string(),
            arguments: arguments.iter().map(|arg| arg.to_string()).collect(),
            stdout_redirection: None,
            stderr_redirection: None,
        }
    }

    #[test]
    fn prints_last_n_entries() {
        let _guard = history_store::test_lock();
        history_store::clear();
        history_store::push("echo one".to_string());
        history_store::push("echo two".to_string());
        history_store::push("echo three".to_string());
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        history(&command(&["2"]), &mut stdout, &mut stderr);

        assert!(stderr.is_empty());
        assert_eq!(
            String::from_utf8(stdout).unwrap(),
            "    2  echo two\n    3  echo three\n"
        );
    }

    #[test]
    fn writes_and_loads_history_files() {
        let _guard = history_store::test_lock();
        history_store::clear();
        history_store::push("pwd".to_string());
        history_store::push("echo hi".to_string());
        let path = temp_file("history");
        let mut stderr = Vec::new();

        write_history_to_file(path.to_str().unwrap(), &mut stderr);
        history_store::clear();
        load_history_from_file(path.to_str().unwrap(), &mut stderr);

        assert!(stderr.is_empty());
        assert_eq!(history_store::get_all(), vec!["pwd", "echo hi"]);

        fs::remove_file(path).unwrap();
    }
}
