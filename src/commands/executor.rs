use crate::commands::built_ins::registry;
use crate::commands::expansion::expand_pipeline;
use crate::commands::external::execute_external_command;
use crate::commands::redirection::open_redirection;
use crate::state::jobs_store;
use crate::syntax::command_invocation::CommandInvocation;
use crate::syntax::pipeline::Pipeline;
use std::io;
use std::io::Write;
use std::process::{Child, Command as ProcessCommand, Stdio};

pub fn execute_pipeline(pipeline: &mut Pipeline) {
    expand_pipeline(pipeline);

    if pipeline.commands.len() == 1 {
        execute_command_invocation(&pipeline.commands[0], pipeline.is_background);
        return;
    }

    execute_pipeline_commands(pipeline);
}

fn execute_pipeline_commands(pipeline: &Pipeline) {
    let command_count = pipeline.commands.len();
    let mut children: Vec<Child> = Vec::new();

    let mut prev_output: Option<Vec<u8>> = None;

    for (i, command_invocation) in pipeline.commands.iter().enumerate() {
        let is_last = i == command_count - 1;

        if registry::is_builtin(&command_invocation.name) {
            let mut buffer: Vec<u8> = Vec::new();

            let mut stdout_writer: Box<dyn Write> = if is_last {
                if let Some(redirection) = &command_invocation.stdout_redirection {
                    Box::new(open_redirection(redirection))
                } else {
                    Box::new(io::stdout())
                }
            } else {
                Box::new(&mut buffer)
            };

            let mut stderr_writer = stderr_writer_for(command_invocation);

            registry::execute(command_invocation, &mut *stdout_writer, &mut *stderr_writer);

            drop(stdout_writer);

            if !is_last {
                prev_output = Some(buffer);
            }
        } else {
            let mut command = ProcessCommand::new(&command_invocation.name);
            command.args(&command_invocation.arguments);

            if prev_output.is_some() {
                command.stdin(Stdio::piped());
            } else if i > 0 {
                let prev_stdout = children.last_mut().unwrap().stdout.take().unwrap();
                command.stdin(prev_stdout);
            } else {
                command.stdin(Stdio::inherit());
            }

            if is_last {
                if let Some(redirection) = &command_invocation.stdout_redirection {
                    command.stdout(open_redirection(redirection));
                } else {
                    command.stdout(Stdio::inherit());
                }
            } else {
                command.stdout(Stdio::piped());
            }

            if let Some(redirection) = &command_invocation.stderr_redirection {
                command.stderr(open_redirection(redirection));
            } else {
                command.stderr(Stdio::inherit());
            }

            match command.spawn() {
                Ok(mut child) => {
                    if let Some(output) = prev_output.take() {
                        let mut child_stdin = child.stdin.take().unwrap();
                        child_stdin.write_all(&output).unwrap();
                        drop(child_stdin);
                    }
                    children.push(child);
                }
                Err(e) => {
                    eprintln!("{}: {}", command_invocation.name, e);
                    return;
                }
            }

            prev_output = None;
        }
    }

    if pipeline.is_background {
        let command_str = pipeline
            .commands
            .iter()
            .map(invocation_to_string)
            .collect::<Vec<_>>()
            .join(" | ");

        let (id, pids) = jobs_store::push_pipeline(children, command_str);
        let pids_str = pids
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("[{}] {}", id, pids_str);
    } else {
        for child in children.iter_mut() {
            child.wait().unwrap();
        }
    }
}

pub fn execute_command_invocation(command_invocation: &CommandInvocation, run_in_background: bool) {
    let mut stdout_writer: Box<dyn Write> =
        if let Some(redirection) = &command_invocation.stdout_redirection {
            Box::new(open_redirection(redirection))
        } else {
            Box::new(io::stdout())
        };

    let mut stderr_writer = stderr_writer_for(command_invocation);

    if !registry::execute(command_invocation, &mut *stdout_writer, &mut *stderr_writer) {
        execute_external_command(command_invocation, run_in_background);
    }
}

fn stderr_writer_for(command_invocation: &CommandInvocation) -> Box<dyn Write> {
    if let Some(redirection) = &command_invocation.stderr_redirection {
        Box::new(open_redirection(redirection))
    } else {
        Box::new(io::stderr())
    }
}

fn invocation_to_string(command_invocation: &CommandInvocation) -> String {
    std::iter::once(command_invocation.name.clone())
        .chain(command_invocation.arguments.iter().cloned())
        .collect::<Vec<_>>()
        .join(" ")
}
