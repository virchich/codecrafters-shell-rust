use crate::commands::command::Command;
use crate::repl::complete_store;
use std::io::Write;

pub fn complete(command: &Command, writer_out: &mut dyn Write, writer_err: &mut dyn Write) {
    if command.arguments.len() > 0 {
        match command.arguments[0].as_str() {
            "-C" => {
                if command.arguments.len() < 3 {
                    writeln!(writer_err, "complete: -C: path and command are required").unwrap();
                    return;
                }

                complete_store::push(command.arguments[1].clone(), command.arguments[2].clone());
            }
            "-p" => {
                if command.arguments.len() < 2 {
                    writeln!(writer_err, "complete: -p: command argument required").unwrap();
                    return;
                }

                let completion_command = &command.arguments[1];

                for record in complete_store::get_all() {
                    if record.command == *completion_command {
                        writeln!(writer_out, "complete -C '{}' {}", record.path, record.command).unwrap();
                        return;
                    }
                }

                writeln!(writer_err, "complete: {}: no completion specification", completion_command).unwrap();
            }
            "-r" => {
                if command.arguments.len() < 2 {
                    writeln!(writer_err, "complete: -r: command argument required").unwrap();
                    return;
                }

                complete_store::remove(command.arguments[1].clone());
            }
            arg => {
                writeln!(writer_err, "complete: unknown argument: {}", arg).unwrap();
            }
        }
    } else {
        writeln!(writer_err, "complete: no arguments").unwrap();
    }
}