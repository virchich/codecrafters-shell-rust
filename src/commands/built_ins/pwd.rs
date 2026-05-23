use crate::syntax::command_invocation::CommandInvocation;
use std::env;
use std::io::Write;

pub fn pwd(command: &CommandInvocation, writer_out: &mut dyn Write, writer_err: &mut dyn Write) {
    match env::current_dir() {
        Ok(output) => {
            writeln!(writer_out, "{}", output.display()).unwrap();
        }
        Err(e) => {
            writeln!(writer_err, "{}: {}", command.name, e).unwrap();
        }
    }
}
