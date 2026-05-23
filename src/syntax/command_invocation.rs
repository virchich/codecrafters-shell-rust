use crate::syntax::redirection::Redirection;

pub struct CommandInvocation {
    pub name: String,
    pub arguments: Vec<String>,
    pub stdout_redirection: Option<Redirection>,
    pub stderr_redirection: Option<Redirection>,
}
