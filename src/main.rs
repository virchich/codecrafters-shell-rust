mod commands;

use commands::core::{read_command, run_command};

fn repl() {
    loop {
        let command = read_command();

        run_command(&command);
    }
}

fn main() {
    repl()
}
