pub fn is_command_allowed(command: &String) -> bool {
    let allowed_commands = ["exit".to_string(), "echo".to_string(), "type".to_string()];

    allowed_commands.contains(command)
}
