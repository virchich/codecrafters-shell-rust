mod commands;

use crate::repl::repl::repl;

mod repl;
pub mod supported_envs;

fn main() {
    repl()
}
