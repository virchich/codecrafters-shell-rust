use crate::commands::statement::{RedirectMode, RedirectStatement};
use crate::commands::validator::is_command_executable;
use std::env::var;
use std::fs::File;
use std::io;
use std::io::Write;

pub fn exec(statement: &RedirectStatement) {
    match var("PATH") {
        Ok(path) => {
            let (executable, _) = is_command_executable(&statement.command.command, path);
            if executable {
                match std::process::Command::new(&statement.command.command)
                    .args(&statement.command.arguments)
                    .output()
                {
                    Ok(output) => {
                        if let Some(redirect) = &statement.redirect_std_out {
                            write_to_file(
                                &redirect.file_location,
                                &redirect.redirect_mode,
                                &output.stdout,
                            );
                        } else {
                            io::stdout().write_all(&output.stdout).unwrap();
                        }

                        if let Some(redirect) = &statement.redirect_std_err {
                            write_to_file(
                                &redirect.file_location,
                                &redirect.redirect_mode,
                                &output.stderr,
                            );
                        } else {
                            io::stderr().write_all(&output.stderr).unwrap();
                        }
                    }
                    Err(e) => {
                        eprintln!("{}: {}", statement.command.command, e);
                    }
                }
            } else {
                eprintln!("{}: not found", statement.command.command);
            }
            return;
        }
        Err(_) => {
            eprintln!("type: PATH variable not set");
            return;
        }
    }
}

fn write_to_file(file_location: &String, write_mode: &RedirectMode, output: &Vec<u8>) {
    match write_mode {
        RedirectMode::Append => {
            let mut file = File::options()
                .create(true)
                .append(true)
                .open(file_location)
                .unwrap();
            file.write_all(&output).unwrap();
        }
        RedirectMode::Overwrite => {
            let mut file = File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(file_location)
                .unwrap();
            file.write_all(&output).unwrap()
        }
    }
}
