use crate::commands::command::Command;
use std::io::Write;

pub fn declare(command: &Command, writer_out: &mut dyn Write, writer_err: &mut dyn Write) {
    if command.arguments.len() > 0 {
        match command.arguments[0].as_str() {
            "-p" => {
                if command.arguments.len() < 2 {
                    writeln!(writer_err, "declare: -p: variable argument is required").unwrap();
                    return;
                }

                writeln!(writer_out, "declare: {}: not found", command.arguments[1]).unwrap();
            }
            arg => {
                writeln!(writer_err, "declare: unknown argument: {}", arg).unwrap();
            }
        }
    } else {
        writeln!(writer_err, "declare: no arguments specified").unwrap();
    }
}
