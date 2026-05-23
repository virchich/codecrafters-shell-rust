use crate::syntax::command_invocation::CommandInvocation;

pub struct Pipeline {
    pub(crate) commands: Vec<CommandInvocation>,
    pub(crate) is_background: bool,
}
