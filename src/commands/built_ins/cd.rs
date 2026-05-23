use crate::syntax::command_invocation::CommandInvocation;
use std::env;
use std::io::Write;

pub fn cd(command: &CommandInvocation, writer_err: &mut dyn Write) {
    let path = match command.arguments.first() {
        None => match env::var("HOME") {
            Ok(home_path) => home_path,
            Err(_) => {
                writeln!(writer_err, "cd: HOME variable not set").unwrap();
                return;
            }
        },
        Some(path) if path == "~" => match env::var("HOME") {
            Ok(home_path) => home_path,
            Err(_) => {
                writeln!(writer_err, "cd: HOME variable not set").unwrap();
                return;
            }
        },
        Some(path) => path.clone(),
    };

    match env::set_current_dir(&path) {
        Ok(_) => {}
        Err(_) => {
            writeln!(writer_err, "cd: {}: No such file or directory", path).unwrap();
        }
    }
}
