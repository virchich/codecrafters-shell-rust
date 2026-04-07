use crate::commands::cmd::cd::cd;
use crate::commands::cmd::echo::echo;
use crate::commands::cmd::exec::exec;
use crate::commands::cmd::exit::exit;
use crate::commands::cmd::pwd::pwd;
use crate::commands::cmd::type_of::type_of;
use crate::commands::statement::{Pipeline, Redirect, RedirectMode, RedirectStatement};
use std::fs::File;
use std::io;
use std::io::Write;
use std::process::{Child, Command, Stdio};

fn open_redirect(redirect: &Redirect) -> File {
    match redirect.redirect_mode {
        RedirectMode::Append => File::options()
            .create(true)
            .append(true)
            .open(&redirect.file_location)
            .unwrap(),
        RedirectMode::Overwrite => File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&redirect.file_location)
            .unwrap(),
    }
}

pub fn run_statement(pipeline: &Pipeline) {
    if pipeline.segments.len() == 1 {
        // Single command — use existing logic (supports builtins + redirects)
        run_redirect(&pipeline.segments[0]);
        return;
    }

    // Multi-segment pipeline — external commands only for now
    run_pipeline(pipeline);
}

fn run_pipeline(pipeline: &Pipeline) {
    let mut children: Vec<Child> = Vec::new();
    let segment_count = pipeline.segments.len();

    for (i, segment) in pipeline.segments.iter().enumerate() {
        let mut cmd = Command::new(&segment.command.command);
        cmd.args(&segment.command.arguments);

        // Set up stdin: first command gets terminal stdin, others get piped from previous
        if i == 0 {
            cmd.stdin(Stdio::inherit());
        } else {
            // Take stdout from the previous child and use it as our stdin
            let prev_stdout = children[i - 1].stdout.take().unwrap();
            cmd.stdin(prev_stdout);
        }

        // Set up stdout: last command goes to terminal (or redirect), others pipe to next
        if i == segment_count - 1 {
            // Last segment — check for stdout redirect
            if let Some(redirect) = &segment.redirect_std_out {
                let file = open_redirect(redirect);
                cmd.stdout(file);
            } else {
                cmd.stdout(Stdio::inherit());
            }
        } else {
            // Not last — pipe stdout to the next command
            cmd.stdout(Stdio::piped());
        }

        // Stderr: check for redirect, otherwise inherit terminal
        if let Some(redirect) = &segment.redirect_std_err {
            let file = open_redirect(redirect);
            cmd.stderr(file);
        } else {
            cmd.stderr(Stdio::inherit());
        }

        match cmd.spawn() {
            Ok(child) => children.push(child),
            Err(e) => {
                eprintln!("{}: {}", segment.command.command, e);
                return;
            }
        }
    }

    // Wait for all children to finish
    for child in children.iter_mut() {
        child.wait().unwrap();
    }
}

pub fn run_redirect(redirect_statement: &RedirectStatement) {
    let mut stdout_writer: Box<dyn Write> =
        if let Some(redirect) = &redirect_statement.redirect_std_out {
            Box::new(open_redirect(redirect))
        } else {
            Box::new(io::stdout())
        };

    let mut stderr_writer: Box<dyn Write> =
        if let Some(redirect) = &redirect_statement.redirect_std_err {
            Box::new(open_redirect(redirect))
        } else {
            Box::new(io::stderr())
        };

    match redirect_statement.command.command.as_str() {
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
        _ => exec(&redirect_statement),
    }
}
