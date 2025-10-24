use crate::commands::command::Command;

pub fn exit(command: &Command) {
    if command.arguments.is_empty() {
        std::process::exit(0);
    }

    match command.arguments[0].parse::<i32>() {
        Ok(code) => std::process::exit(code),
        Err(_) => {
            eprintln!("exit: {}: numeric argument required", command.arguments[0]);
            std::process::exit(255);
        }
    }
}
