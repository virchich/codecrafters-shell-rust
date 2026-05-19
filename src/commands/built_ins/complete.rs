use crate::commands::command::Command;
use std::io::Write;

pub fn complete(command: &Command, _writer_out: &mut dyn Write, writer_err: &mut dyn Write) {
    if command.arguments.len() > 0 {
        match command.arguments[0].as_str() {
            "-p" => {
                if command.arguments.len() < 2 {
                    writeln!(writer_err, "complete: -p: command argument required").unwrap();
                    return;
                }
                writeln!(writer_err, "complete: {}: no completion specification", command.arguments[1]).unwrap();
            }
            arg => {
                writeln!(writer_err, "complete: unknown argument: {}", arg).unwrap();
            }
        }
    } else {
        writeln!(writer_err, "complete: no arguments").unwrap();
    }
}