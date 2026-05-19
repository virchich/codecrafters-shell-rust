use crate::commands::built_ins::jobs::print_done_jobs;
use crate::commands::core::run_statement;
use crate::commands::parser::parser::Parser;
use crate::commands::scanner::lexer::Lexer;
use crate::repl::editor::get_editor;
use crate::repl::history_store;
use std::io;

pub fn repl() {
    let mut editor = get_editor();

    loop {
        print_done_jobs(&mut Box::new(io::stdout()));

        let line = editor.readline("$ ");

        match line {
            Ok(line) => {
                let _ = editor.add_history_entry(line.clone());
                history_store::push(line.clone());

                let scanner = Lexer::new(line);
                let tokens = scanner.scan_tokens();

                let mut parser = Parser::new(tokens);

                let statement = parser.parse();

                match statement {
                    Some(command) => {
                        run_statement(&command);
                    }
                    None => continue,
                }
            }
            Err(_) => continue,
        }
    }
}
