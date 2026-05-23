use crate::commands::command::Command;
use crate::repl::declare_store;
use std::io::Write;

pub fn declare(command: &Command, writer_out: &mut dyn Write, writer_err: &mut dyn Write) {
    if command.arguments.len() > 0 {
        match command.arguments[0].as_str() {
            "-p" => {
                if command.arguments.len() < 2 {
                    writeln!(writer_err, "declare: -p: variable argument is required").unwrap();
                    return;
                }

                let optional_value_in_store = declare_store::get(command.arguments[1].to_string());

                if let Some(variable_value) = optional_value_in_store {
                    writeln!(
                        writer_out,
                        "declare -- {}=\"{}\"",
                        command.arguments[1].to_string(),
                        variable_value
                    )
                        .unwrap();
                } else {
                    writeln!(writer_err, "declare: {}: not found", command.arguments[1]).unwrap();
                }
            }
            arg => {
                let declare_argument = arg.split("=").collect::<Vec<&str>>();

                if declare_argument.len() < 2 {
                    writeln!(writer_err, "declare: declaration requires two arguments: <VARIABLE>=<VALUE>").unwrap();
                    return;
                }

                declare_store::add(declare_argument[0].to_string(), declare_argument[1].to_string());
            }
        }
    } else {
        writeln!(writer_err, "declare: no arguments specified").unwrap();
    }
}
