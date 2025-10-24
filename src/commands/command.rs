use crate::commands::validator::is_command_allowed;

pub struct Command {
    pub command: String,
    pub arguments: Vec<String>,
}
