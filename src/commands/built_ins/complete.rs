use crate::state::complete_store;
use crate::syntax::command_invocation::CommandInvocation;
use std::io::Write;

pub fn complete(
    command: &CommandInvocation,
    writer_out: &mut dyn Write,
    writer_err: &mut dyn Write,
) {
    if !command.arguments.is_empty() {
        match command.arguments[0].as_str() {
            "-C" => {
                if command.arguments.len() < 3 {
                    writeln!(writer_err, "complete: -C: path and command are required").unwrap();
                    return;
                }

                complete_store::push(command.arguments[1].clone(), command.arguments[2].clone());
            }
            "-p" => {
                if command.arguments.len() < 2 {
                    writeln!(writer_err, "complete: -p: command argument required").unwrap();
                    return;
                }

                let completion_command = &command.arguments[1];

                for record in complete_store::get_all() {
                    if record.command == *completion_command {
                        writeln!(
                            writer_out,
                            "complete -C '{}' {}",
                            record.path, record.command
                        )
                        .unwrap();
                        return;
                    }
                }

                writeln!(
                    writer_err,
                    "complete: {}: no completion specification",
                    completion_command
                )
                .unwrap();
            }
            "-r" => {
                if command.arguments.len() < 2 {
                    writeln!(writer_err, "complete: -r: command argument required").unwrap();
                    return;
                }

                complete_store::remove(command.arguments[1].clone());
            }
            arg => {
                writeln!(writer_err, "complete: unknown argument: {}", arg).unwrap();
            }
        }
    } else {
        writeln!(writer_err, "complete: no arguments").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::complete;
    use crate::state::complete_store;
    use crate::syntax::command_invocation::CommandInvocation;

    fn command(arguments: &[&str]) -> CommandInvocation {
        CommandInvocation {
            name: "complete".to_string(),
            arguments: arguments.iter().map(|arg| arg.to_string()).collect(),
            stdout_redirection: None,
            stderr_redirection: None,
        }
    }

    #[test]
    fn adds_prints_and_removes_completion_specs() {
        let _guard = complete_store::test_lock();
        complete_store::clear();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        complete(&command(&["-C", "/tmp/spec", "git"]), &mut stdout, &mut stderr);
        complete(&command(&["-p", "git"]), &mut stdout, &mut stderr);
        complete(&command(&["-r", "git"]), &mut stdout, &mut stderr);

        assert!(stderr.is_empty());
        assert_eq!(
            String::from_utf8(stdout).unwrap(),
            "complete -C '/tmp/spec' git\n"
        );
        assert!(complete_store::get_all().is_empty());
    }

    #[test]
    fn reports_missing_arguments() {
        let _guard = complete_store::test_lock();
        complete_store::clear();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        complete(&command(&[]), &mut stdout, &mut stderr);

        assert!(stdout.is_empty());
        assert_eq!(String::from_utf8(stderr).unwrap(), "complete: no arguments\n");
    }
}
