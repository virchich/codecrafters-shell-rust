mod commands;

use crate::repl::repl::repl;

mod repl;
pub mod supported_envs;
pub mod syntax;
pub mod state;

fn main() {
    repl()
}
