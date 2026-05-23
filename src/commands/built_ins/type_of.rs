use crate::commands::built_ins::registry::is_builtin;
use crate::commands::validator::is_command_executable;
use crate::supported_envs::SupportedEnv;
use crate::syntax::command_invocation::CommandInvocation;
use std::env::var;
use std::io::Write;

pub fn type_of(
    command: &CommandInvocation,
    writer_out: &mut dyn Write,
    writer_err: &mut dyn Write,
) {
    if command.arguments.is_empty() || command.arguments.len() > 1 {
        writeln!(writer_err, "type: must provide one argument").unwrap();
        return;
    }

    let command_argument = command.arguments.first().unwrap();

    if is_builtin(command_argument) {
        writeln!(writer_out, "{} is a shell builtin", command_argument).unwrap();
        return;
    }

    match var(SupportedEnv::PATH) {
        Ok(path) => {
            let (executable, executable_path) = is_command_executable(command_argument, path);
            if executable {
                writeln!(writer_out, "{} is {}", command_argument, executable_path).unwrap();
            } else {
                writeln!(writer_err, "{}: not found", command_argument).unwrap();
            }
        }
        Err(_) => {
            writeln!(writer_err, "type: PATH variable not set").unwrap();
        }
    }
}
