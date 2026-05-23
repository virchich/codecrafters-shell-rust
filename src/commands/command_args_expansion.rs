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
        let mut str_builder = String::new();

        for (byte_index, ch) in arg.char_indices() {
            if ch != '$' || byte_index == arg.len() - 1 {
                str_builder.push(ch);
                continue;
            }

            match declare_store::get(&arg[byte_index + 1..]) {
                Some(variable_value) => str_builder.push_str(&variable_value),
                None => str_builder.push_str(""),
            }

            break;
        }

        *arg = str_builder;
    }
}
