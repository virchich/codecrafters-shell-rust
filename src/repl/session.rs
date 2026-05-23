use crate::commands::built_ins::jobs::print_done_jobs;
use crate::commands::executor::execute_pipeline;
use crate::repl::editor::get_editor;
use crate::state::history_store;
use crate::syntax::parser::Parser;
use crate::syntax::scanner::lexer::Lexer;
use std::io;

pub fn run() {
    let mut editor = get_editor();

    loop {
        print_done_jobs(&mut Box::new(io::stdout()));

        let line = editor.readline("$ ");

        match line {
            Ok(line) => {
                let _ = editor.add_history_entry(line.clone());
                history_store::push(line.clone());

                let scanner = Lexer::new(line);
                let tokens = match scanner.scan_tokens() {
                    Ok(tokens) => tokens,
                    Err(error) => {
                        eprintln!("{}", error);
                        continue;
                    }
                };

                let mut parser = Parser::new(tokens);
                let pipeline = match parser.parse() {
                    Ok(pipeline) => pipeline,
                    Err(error) => {
                        eprintln!("{}", error);
                        continue;
                    }
                };

                if let Some(mut pipeline) = pipeline {
                    execute_pipeline(&mut pipeline);
                }
            }
            Err(_) => continue,
        }
    }
}
