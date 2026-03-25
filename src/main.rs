mod commands;

use commands::core::{read_line, run_statement};

fn repl() {
    loop {
        let statement = read_line();

        match statement {
            Some(command) => { run_statement(&command); }
            None => continue,
        }
    }
}

fn main() {
    repl()
}
