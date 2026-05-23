use crate::commands::redirection::open_redirection;
use crate::commands::validator::is_command_executable;
use crate::state::jobs_store;
use crate::supported_envs::SupportedEnv;
use crate::syntax::command_invocation::CommandInvocation;
use std::env::var;
use std::process::Stdio;

pub fn execute_external_command(command_invocation: &CommandInvocation, run_in_background: bool) {
    match var(SupportedEnv::PATH) {
        Ok(path) => {
            let (executable, _) = is_command_executable(&command_invocation.name, path);
            if executable {
                let mut command = std::process::Command::new(&command_invocation.name);
                command.args(&command_invocation.arguments);

                if let Some(redirection) = &command_invocation.stdout_redirection {
                    command.stdout(Stdio::from(open_redirection(redirection)));
                }

                if let Some(redirection) = &command_invocation.stderr_redirection {
                    command.stderr(Stdio::from(open_redirection(redirection)));
                }

                let mut child = command.spawn().unwrap();

                if run_in_background {
                    let command_str = std::iter::once(command_invocation.name.clone())
                        .chain(command_invocation.arguments.iter().cloned())
                        .collect::<Vec<_>>()
                        .join(" ");
                    let (job_id, job_pid) = jobs_store::push(child, command_str);
                    println!("[{}] {}", job_id, job_pid);
                } else {
                    child.wait().unwrap();
                }
            } else {
                eprintln!("{}: not found", command_invocation.name);
            }
        }
        Err(_) => {
            eprintln!("type: PATH variable not set");
        }
    }
}
