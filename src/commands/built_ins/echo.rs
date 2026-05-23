use crate::syntax::command_invocation::CommandInvocation;
use std::io::Write;

pub fn echo(command: &CommandInvocation, writer: &mut dyn Write) {
    let output = command.arguments.join(" ");
    writeln!(writer, "{}", output).unwrap();
}
