use crate::state::declare_store;
use crate::syntax::command_invocation::CommandInvocation;
use regex::Regex;
use std::io::Write;

pub fn declare(
    command: &CommandInvocation,
    writer_out: &mut dyn Write,
    writer_err: &mut dyn Write,
) {
    if !command.arguments.is_empty() {
        match command.arguments[0].as_str() {
            "-p" => {
                if command.arguments.len() < 2 {
                    writeln!(writer_err, "declare: -p: variable argument is required").unwrap();
                    return;
                }

                let optional_value_in_store = declare_store::get(command.arguments[1].as_str());

                if let Some(variable_value) = optional_value_in_store {
                    writeln!(
                        writer_out,
                        "declare -- {}=\"{}\"",
                        command.arguments[1], variable_value
                    )
                    .unwrap();
                } else {
                    writeln!(writer_err, "declare: {}: not found", command.arguments[1]).unwrap();
                }
            }
            arg => {
                let declare_argument = arg.split("=").collect::<Vec<&str>>();

                if declare_argument.len() < 2 {
                    writeln!(
                        writer_err,
                        "declare: declaration requires two arguments: <VARIABLE>=<VALUE>"
                    )
                    .unwrap();
                    return;
                }

                let variable_name = declare_argument[0];
                let variable_value = declare_argument[1];

                let re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();

                if re.is_match(variable_name) {
                    declare_store::add(variable_name.to_string(), variable_value.to_string());
                } else {
                    writeln!(writer_err, "declare: `{}': not a valid identifier", arg).unwrap();
                }
            }
        }
    } else {
        writeln!(writer_err, "declare: no arguments specified").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::declare;
    use crate::state::declare_store;
    use crate::syntax::command_invocation::CommandInvocation;

    fn command(arguments: &[&str]) -> CommandInvocation {
        CommandInvocation {
            name: "declare".to_string(),
            arguments: arguments.iter().map(|arg| arg.to_string()).collect(),
            stdout_redirection: None,
            stderr_redirection: None,
        }
    }

    #[test]
    fn stores_and_prints_declared_variables() {
        let _guard = declare_store::test_lock();
        declare_store::clear();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        declare(&command(&["FOO=bar"]), &mut stdout, &mut stderr);
        declare(&command(&["-p", "FOO"]), &mut stdout, &mut stderr);

        assert!(stderr.is_empty());
        assert_eq!(
            String::from_utf8(stdout).unwrap(),
            "declare -- FOO=\"bar\"\n"
        );
        assert_eq!(declare_store::get("FOO"), Some("bar".to_string()));
    }

    #[test]
    fn rejects_invalid_identifiers() {
        let _guard = declare_store::test_lock();
        declare_store::clear();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        declare(&command(&["1BAD=value"]), &mut stdout, &mut stderr);

        assert!(stdout.is_empty());
        assert_eq!(
            String::from_utf8(stderr).unwrap(),
            "declare: `1BAD=value': not a valid identifier\n"
        );
    }
}
