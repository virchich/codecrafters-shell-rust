mod commands;

use commands::core::{read_command, run_command};

fn repl() {
    loop {
        let command = read_command();

        match command {
            Some(command) => { run_command(&command); }
            None => continue,
        }
    }
}

fn main() {
    repl()
}
