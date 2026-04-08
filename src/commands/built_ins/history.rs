use crate::repl::history_store;
use std::io::Write;

pub fn history(output: &mut dyn Write) {
    let entries = history_store::get_all();

    for (i, command) in entries.iter().enumerate() {
        writeln!(output, "  {} {}", i + 1, command).unwrap();
    }
}
