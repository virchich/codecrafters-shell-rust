use crate::repl::repl_helper::ReplHelper;
use rustyline::config::BellStyle;
use rustyline::history::DefaultHistory;
use rustyline::{CompletionType, Config, Editor};

pub fn get_editor() -> Editor<ReplHelper, DefaultHistory> {
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .bell_style(BellStyle::Audible)
        .max_history_size(100)
        .unwrap()
        .build();

    let mut editor: Editor<ReplHelper, _> = Editor::with_config(config).unwrap();
    editor.set_helper(Some(ReplHelper::new()));
    editor
}
