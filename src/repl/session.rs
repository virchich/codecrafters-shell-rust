use crate::commands::built_ins::jobs::print_done_jobs;
use crate::repl::editor::get_editor;
use crate::shell::execute_line;
use rustyline::error::ReadlineError;
use std::io;

pub fn run() {
    let mut editor = get_editor();

    loop {
        print_done_jobs(&mut Box::new(io::stdout()));

        let line = editor.readline("$ ");

        match line {
            Ok(line) => {
                let _ = editor.add_history_entry(line.clone());
                execute_line(line);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::WindowResized) => continue,
            Err(ReadlineError::Eof) => break,
            Err(error) => {
                eprintln!("readline: {}", error);
                break;
            }
        }
    }
}
