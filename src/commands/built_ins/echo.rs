use crate::syntax::command_invocation::CommandInvocation;
use std::io::Write;

pub fn echo(command: &CommandInvocation, writer: &mut dyn Write, _writer_err: &mut dyn Write) {
    let output = command.arguments.join(" ");
    writeln!(writer, "{}", output).unwrap();
}

#[cfg(test)]
mod tests {
    use super::echo;
    use crate::syntax::command_invocation::CommandInvocation;

    #[test]
    fn joins_arguments_with_spaces_and_trailing_newline() {
        let command = CommandInvocation {
            name: "echo".to_string(),
            arguments: vec!["hello".to_string(), "world".to_string()],
            stdout_redirection: None,
            stderr_redirection: None,
        };
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        echo(&command, &mut stdout, &mut stderr);

        assert_eq!(String::from_utf8(stdout).unwrap(), "hello world\n");
        assert!(stderr.is_empty());
    }
}
