mod commands;

mod repl;
mod shell;
pub mod state;
pub mod supported_envs;
pub mod syntax;

fn main() {
    repl::session::run()
}
