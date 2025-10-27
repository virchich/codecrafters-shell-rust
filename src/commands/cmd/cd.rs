use std::env;
use crate::commands::command::Command;

pub fn cd(command: &Command) {
    let mut path: String = "~".to_string();
    if command.arguments.is_empty() || (command.arguments.len() >= 1 && command.arguments[0] == "~"){
        match env::var("HOME") {
            Ok(home_path) => {
                path = home_path;
            }
            Err(_) => {
                eprintln!("cd: HOME variable not set");
                return;
            }
        }
    } else if command.arguments.len() >= 1 {
        path = command.arguments[0].clone();
    }

    match env::set_current_dir(&path) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("cd: {}: No such file or directory", path);
        }
    }
}