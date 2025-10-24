use crate::commands::command::Command;

pub fn is_command_allowed(command: &Command) -> bool {
    let allowed_commands = [
        "exit".to_string(),
        "echo".to_string(),
    ];

    allowed_commands.contains(&command.command)
}