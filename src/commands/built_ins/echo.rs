use crate::commands::command::Command;
use std::io::Write;

pub fn echo(command: &Command, writer: &mut dyn Write) {
    let output = command.arguments.join(" ");
    writeln!(writer, "{}", output).unwrap();
}
