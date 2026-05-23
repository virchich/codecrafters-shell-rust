use crate::commands::built_ins::history::load_history_from_file;
use crate::repl::repl_helper::ReplHelper;
use crate::supported_envs::SupportedEnv;
use rustyline::config::BellStyle;
use rustyline::history::DefaultHistory;
use rustyline::{CompletionType, Config, Editor};
use std::{env, fs};

pub fn get_editor() -> Editor<ReplHelper, DefaultHistory> {
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .bell_style(BellStyle::Audible)
        .max_history_size(100)
        .unwrap()
        .build();

    let mut editor: Editor<ReplHelper, _> = Editor::with_config(config).unwrap();
    editor.set_helper(Some(ReplHelper::new()));

    if let Ok(path) = env::var(SupportedEnv::HISTFILE) {
        load_history_from_file_on_startup(path.as_str(), &mut editor);
    }

    editor
}

fn load_history_from_file_on_startup(
    history_file: &str,
    editor: &mut Editor<ReplHelper, DefaultHistory>,
) {
    match fs::read_to_string(history_file) {
        Ok(contents) => {
            for line in contents.lines() {
                editor.add_history_entry(line.to_string()).unwrap();
            }

            load_history_from_file(history_file, &mut std::io::stderr());
        }
        Err(e) => {
            eprintln!("Failed to load history from file {}: {}", history_file, e);
        }
    }
}
