mod commands;

mod repl;
pub mod state;
pub mod supported_envs;
pub mod syntax;

fn main() {
    repl::session::run()
}
