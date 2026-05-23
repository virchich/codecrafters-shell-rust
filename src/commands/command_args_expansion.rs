use crate::commands::command::Command;
use crate::commands::statement::Pipeline;
use crate::repl::declare_store;

pub fn command_args_expansion(pipeline: &mut Pipeline) {
    pipeline
        .segments
        .iter_mut()
        .for_each(|redirect_stmt| expand_command_arguments(&mut redirect_stmt.command))
}

fn expand_command_arguments(command: &mut Command) {
    for arg in &mut command.arguments {
        if arg.starts_with('$') && arg.len() > 1 {
            match declare_store::get(&arg[1..]) {
                Some(variable_value) => *arg = variable_value,
                None => *arg = String::from(""),
            }
        }
    }
}
