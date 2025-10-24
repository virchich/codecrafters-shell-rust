use crate::commands::command::Command;
use crate::commands::validator::is_command_executable;
use std::env::var;
use std::io;
use std::io::Write;

pub fn exec(command: &Command) {
    match var("PATH") {
        Ok(path) => {
            let (executable, _) = is_command_executable(&command.command, path);
            if executable {
                match std::process::Command::new(&command.command)
                    .args(&command.arguments)
                    .output()
                {
                    Ok(output) => {
                        io::stdout().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();
                    }
                    Err(e) => {
                        eprintln!("{}: {}", command.command, e);
                    }
                }
            } else {
                eprintln!("{}: not found", command.command);
            }
            return;
        }
        Err(_) => {
            eprintln!("type: PATH variable not set");
            return;
        }
    }
}
