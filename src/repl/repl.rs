use crate::commands::built_ins::jobs::print_done_jobs;
use crate::commands::executor::run_statement;
use crate::repl::editor::get_editor;
use crate::state::history_store;
use crate::syntax::parser::parser::Parser;
use crate::syntax::scanner::lexer::Lexer;
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
                    Some(mut command) => {
                        run_statement(&mut command);
                    }
                    None => continue,
                }
            }
            Err(_) => continue,
        }
    }
}
