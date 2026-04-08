use std::env;
use std::io::Write;
use crate::commands::command::Command;

pub fn cd(command: &Command, writer_err: &mut dyn Write) {
    let mut path: String = "~".to_string();
    if command.arguments.is_empty() || (command.arguments.len() >= 1 && command.arguments[0] == "~"){
        match env::var("HOME") {
            Ok(home_path) => {
                path = home_path;
            }
            Err(_) => {
                writeln!(writer_err, "cd: HOME variable not set").unwrap();
                return;
            }
        }
    } else if command.arguments.len() >= 1 {
        path = command.arguments[0].clone();
    }

    match env::set_current_dir(&path) {
        Ok(_) => {}
        Err(_) => {
            writeln!(writer_err, "cd: {}: No such file or directory", path).unwrap();
        }
    }
}