use crate::commands::statement::RedirectStatement;
use crate::commands::utils::open_redirect;
use crate::commands::validator::is_command_executable;
use crate::repl::jobs_store;
use crate::supported_envs::SupportedEnv;
use std::env::var;
use std::process::Stdio;

pub fn exec(statement: &RedirectStatement, run_in_background: bool) {
    match var(SupportedEnv::PATH) {
        Ok(path) => {
            let (executable, _) = is_command_executable(&statement.command.command, path);
            if executable {
                let mut command = std::process::Command::new(&statement.command.command);
                command.args(&statement.command.arguments);

                if let Some(redirect) = &statement.redirect_std_out {
                    command.stdout(Stdio::from(open_redirect(redirect)));
                }

                if let Some(redirect) = &statement.redirect_std_err {
                    command.stderr(Stdio::from(open_redirect(redirect)));
                }

                let mut child = command.spawn().unwrap();

                if run_in_background {
                    let command_str = std::iter::once(statement.command.command.clone())
                        .chain(statement.command.arguments.iter().cloned())
                        .collect::<Vec<_>>()
                        .join(" ");
                    let (job_id, job_pid) = jobs_store::push(child, command_str);
                    println!("[{}] {}", job_id, job_pid);
                } else {
                    child.wait().unwrap();
                }
            } else {
                eprintln!("{}: not found", statement.command.command);
            }
            return;
        }
        Err(_) => {
            eprintln!("type: PATH variable not set");
            return;
        }
    }
}
