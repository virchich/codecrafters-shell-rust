use crate::commands::command::Command;

pub fn echo(command: &Command) {
    let output = command.arguments.join(" ");
    println!("{}", output);
}
