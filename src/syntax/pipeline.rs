use crate::syntax::command_invocation::CommandInvocation;

#[derive(Debug)]
pub struct Pipeline {
    pub(crate) commands: Vec<CommandInvocation>,
    pub(crate) is_background: bool,
}
