use crate::commands::built_ins::cd::cd;
use crate::commands::built_ins::complete::complete;
use crate::commands::built_ins::declare::declare;
use crate::commands::built_ins::echo::echo;
use crate::commands::built_ins::exit::exit;
use crate::commands::built_ins::history::history;
use crate::commands::built_ins::jobs::jobs;
use crate::commands::built_ins::pwd::pwd;
use crate::commands::built_ins::type_of::type_of;
use crate::syntax::command_invocation::CommandInvocation;
use std::io::Write;

type BuiltinHandler = fn(&CommandInvocation, &mut dyn Write, &mut dyn Write);

struct Builtin {
    name: &'static str,
    handler: BuiltinHandler,
}

const BUILTINS: &[Builtin] = &[
    Builtin {
        name: "cd",
        handler: cd,
    },
    Builtin {
        name: "complete",
        handler: complete,
    },
    Builtin {
        name: "declare",
        handler: declare,
    },
    Builtin {
        name: "echo",
        handler: echo,
    },
    Builtin {
        name: "exit",
        handler: exit,
    },
    Builtin {
        name: "history",
        handler: history,
    },
    Builtin {
        name: "jobs",
        handler: jobs,
    },
    Builtin {
        name: "pwd",
        handler: pwd,
    },
    Builtin {
        name: "type",
        handler: type_of,
    },
];

pub fn names() -> impl Iterator<Item = &'static str> {
    BUILTINS.iter().map(|builtin| builtin.name)
}

pub fn is_builtin(command_name: &str) -> bool {
    find(command_name).is_some()
}

pub fn execute(
    command: &CommandInvocation,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
) -> bool {
    if let Some(builtin) = find(&command.name) {
        (builtin.handler)(command, stdout, stderr);
        true
    } else {
        false
    }
}

fn find(command_name: &str) -> Option<&'static Builtin> {
    BUILTINS.iter().find(|builtin| builtin.name == command_name)
}

#[cfg(test)]
mod tests {
    use super::{execute, is_builtin, names};
    use crate::syntax::command_invocation::CommandInvocation;

    #[test]
    fn exposes_known_builtin_names() {
        let builtin_names: Vec<&str> = names().collect();

        assert!(builtin_names.contains(&"echo"));
        assert!(builtin_names.contains(&"type"));
        assert!(is_builtin("pwd"));
        assert!(!is_builtin("definitely-not-a-builtin"));
    }

    #[test]
    fn executes_builtin_and_reports_success() {
        let command = CommandInvocation {
            name: "echo".to_string(),
            arguments: vec!["hi".to_string()],
            stdout_redirection: None,
            stderr_redirection: None,
        };
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let was_executed = execute(&command, &mut stdout, &mut stderr);

        assert!(was_executed);
        assert_eq!(String::from_utf8(stdout).unwrap(), "hi\n");
        assert!(stderr.is_empty());
    }
}
