use crate::commands::built_ins::history::write_history_to_file;
use crate::supported_envs::SupportedEnv;
use crate::syntax::command_invocation::CommandInvocation;
use std::env;
use std::io::Write;

pub fn exit(command: &CommandInvocation, _writer_out: &mut dyn Write, writer_err: &mut dyn Write) {
    if let Ok(path) = env::var(SupportedEnv::HISTFILE) {
        write_history_to_file(&path, writer_err);
    }

    if command.arguments.is_empty() {
        std::process::exit(0);
    }

    match command.arguments[0].parse::<i32>() {
        Ok(code) => std::process::exit(code),
        Err(_) => {
            writer_err
                .write_all(
                    format!("exit: {}: numeric argument required", command.arguments[0]).as_bytes(),
                )
                .unwrap();
            std::process::exit(255);
        }
    }
}
