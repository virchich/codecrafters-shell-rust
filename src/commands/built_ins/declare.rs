use crate::commands::command::Command;
use crate::repl::declare_store;
use regex::Regex;
use std::io::Write;

pub fn declare(command: &Command, writer_out: &mut dyn Write, writer_err: &mut dyn Write) {
    if command.arguments.len() > 0 {
        match command.arguments[0].as_str() {
            "-p" => {
                if command.arguments.len() < 2 {
                    writeln!(writer_err, "declare: -p: variable argument is required").unwrap();
                    return;
                }

                let optional_value_in_store = declare_store::get(command.arguments[1].as_str());

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

                let variable_name = declare_argument[0];
                let variable_value = declare_argument[1];

                let re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();

                if re.is_match(variable_name) {
                    declare_store::add(variable_name.to_string(), variable_value.to_string());
                } else {
                    writeln!(writer_err, "declare: `{}': not a valid identifier", arg).unwrap();
                }
            }
        }
    } else {
        writeln!(writer_err, "declare: no arguments specified").unwrap();
    }
}
