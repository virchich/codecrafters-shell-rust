use crate::commands::built_ins::cd::cd;
use crate::commands::built_ins::complete::complete;
use crate::commands::built_ins::declare::declare;
use crate::commands::built_ins::echo::echo;
use crate::commands::built_ins::exec::exec;
use crate::commands::built_ins::exit::exit;
use crate::commands::built_ins::history::history;
use crate::commands::built_ins::jobs::jobs;
use crate::commands::built_ins::pwd::pwd;
use crate::commands::built_ins::type_of::type_of;
use crate::commands::command_args_expansion::command_args_expansion;
use crate::commands::utils::open_redirect;
use crate::commands::validator::is_command_built_in;
use crate::state::jobs_store;
use crate::syntax::statement::{Pipeline, RedirectStatement};
use std::io;
use std::io::Write;
use std::process::{Child, Command, Stdio};

pub fn run_statement(pipeline: &mut Pipeline) {
    command_args_expansion(pipeline);

    if pipeline.segments.len() == 1 {
        // Single command — use existing logic (supports builtins + redirects)
        run_redirect(&pipeline.segments[0], pipeline.is_background);
        return;
    }

    // Multi-segment pipeline — external commands only for now
    run_pipeline(pipeline);
}

fn run_pipeline(pipeline: &Pipeline) {
    let segment_count = pipeline.segments.len();
    let mut children: Vec<Child> = Vec::new();

    // Tracks the output from the previous segment.
    // None for the first segment (reads from terminal).
    // Some(bytes) for subsequent segments (fed from previous output).
    let mut prev_output: Option<Vec<u8>> = None;

    for (i, segment) in pipeline.segments.iter().enumerate() {
        let is_last = i == segment_count - 1;

        if is_command_built_in(&segment.command.command) {
            // --- Builtin command ---
            // Run it with a buffer as stdout writer.
            let mut buffer: Vec<u8> = Vec::new();

            // Determine where to write: if last segment with redirect, use file;
            // if last segment without redirect, use real stdout;
            // otherwise capture into buffer for next segment.
            let mut stdout_writer: Box<dyn Write> = if is_last {
                if let Some(redirect) = &segment.redirect_std_out {
                    Box::new(open_redirect(redirect))
                } else {
                    Box::new(io::stdout())
                }
            } else {
                Box::new(&mut buffer)
            };

            let mut stderr_writer: Box<dyn Write> = get_redirect(&segment);

            match segment.command.command.as_str() {
                "history" => history(&segment.command, &mut *stdout_writer, &mut *stderr_writer),
                "exit" => exit(&segment.command, &mut *stderr_writer),
                "echo" => echo(&segment.command, &mut *stdout_writer),
                "type" => type_of(&segment.command, &mut *stdout_writer, &mut *stderr_writer),
                "pwd" => pwd(&segment.command, &mut *stdout_writer, &mut *stderr_writer),
                "cd" => cd(&segment.command, &mut *stderr_writer),
                "jobs" => jobs(&segment.command, &mut *stdout_writer),
                "complete" => complete(&segment.command, &mut *stdout_writer, &mut *stderr_writer),
                "declare" => declare(&segment.command, &mut *stdout_writer, &mut *stderr_writer),
                _ => {}
            }

            // Drop writer so buffer is no longer borrowed
            drop(stdout_writer);

            if !is_last {
                prev_output = Some(buffer);
            }
        } else {
            // --- External command ---
            let mut cmd = Command::new(&segment.command.command);
            cmd.args(&segment.command.arguments);

            // Stdin: first segment inherits terminal, others get previous output
            if let Some(_) = prev_output {
                // Previous segment was a builtin — pipe it's buffer in
                cmd.stdin(Stdio::piped());
            } else if i > 0 {
                // Previous segment was an external — take its stdout
                let prev_stdout = children.last_mut().unwrap().stdout.take().unwrap();
                cmd.stdin(prev_stdout);
            } else {
                cmd.stdin(Stdio::inherit());
            }

            // Stdout: last goes to terminal/redirect, others pipe forward
            if is_last {
                if let Some(redirect) = &segment.redirect_std_out {
                    cmd.stdout(open_redirect(redirect));
                } else {
                    cmd.stdout(Stdio::inherit());
                }
            } else {
                cmd.stdout(Stdio::piped());
            }

            // Stderr
            if let Some(redirect) = &segment.redirect_std_err {
                cmd.stderr(open_redirect(redirect));
            } else {
                cmd.stderr(Stdio::inherit());
            }

            match cmd.spawn() {
                Ok(mut child) => {
                    // If previous was a builtin, write its output to this child's stdin
                    if let Some(output) = prev_output.take() {
                        let mut child_stdin = child.stdin.take().unwrap();
                        child_stdin.write_all(&output).unwrap();
                        drop(child_stdin); // close stdin so child knows input is done
                    }
                    children.push(child);
                }
                Err(e) => {
                    eprintln!("{}: {}", segment.command.command, e);
                    return;
                }
            }

            prev_output = None;
        }
    }

    if pipeline.is_background {
        let command_str = pipeline
            .segments
            .iter()
            .map(|s| {
                std::iter::once(s.command.command.clone())
                    .chain(s.command.arguments.iter().cloned())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
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
        // Wait for all external children to finish
        for child in children.iter_mut() {
            child.wait().unwrap();
        }
    }
}

pub fn run_redirect(redirect_statement: &RedirectStatement, run_in_background: bool) {
    let mut stdout_writer: Box<dyn Write> =
        if let Some(redirect) = &redirect_statement.redirect_std_out {
            Box::new(open_redirect(redirect))
        } else {
            Box::new(io::stdout())
        };

    let mut stderr_writer = get_redirect(&redirect_statement);

    match redirect_statement.command.command.as_str() {
        "history" => history(
            &redirect_statement.command,
            &mut *stdout_writer,
            &mut *stderr_writer,
        ),
        "exit" => exit(&redirect_statement.command, &mut *stderr_writer),
        "echo" => echo(&redirect_statement.command, &mut *stdout_writer),
        "type" => type_of(
            &redirect_statement.command,
            &mut *stdout_writer,
            &mut *stderr_writer,
        ),
        "pwd" => pwd(
            &redirect_statement.command,
            &mut *stdout_writer,
            &mut *stderr_writer,
        ),
        "cd" => cd(&redirect_statement.command, &mut *stderr_writer),
        "jobs" => jobs(&redirect_statement.command, &mut *stdout_writer),
        "complete" => complete(
            &redirect_statement.command,
            &mut *stdout_writer,
            &mut stderr_writer,
        ),
        "declare" => declare(
            &redirect_statement.command,
            &mut *stdout_writer,
            &mut *stderr_writer,
        ),
        _ => exec(&redirect_statement, run_in_background),
    }
}

fn get_redirect(redirect_statement: &&RedirectStatement) -> Box<dyn Write> {
    let stderr_writer: Box<dyn Write> = if let Some(redirect) = &redirect_statement.redirect_std_err
    {
        Box::new(open_redirect(redirect))
    } else {
        Box::new(io::stderr())
    };
    stderr_writer
}
