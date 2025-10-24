use crate::commands::validator::is_command_allowed;

pub struct Command {
    pub command: String,
    pub arguments: Vec<String>,
}

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

pub fn echo(command: &Command) {
    let output = command.arguments.join(" ");
    println!("{}", output);
}

pub fn type_of(command: &Command) {
    if command.arguments.is_empty() || command.arguments.len() > 1 {
        eprintln!("type: must provide one argument");
        return;
    }

    if !is_command_allowed(command.arguments.first().unwrap()) {
        eprintln!("{}: not found", command.arguments.first().unwrap());
        return;
    }

    println!("{} is a shell builtin", command.arguments.first().unwrap());
}
