use crate::commands::executor::execute_pipeline;
use crate::state::history_store;
use crate::syntax::parser::Parser;
use crate::syntax::scanner::lexer::Lexer;

pub fn execute_line(line: String) {
    history_store::push(line.clone());

    let scanner = Lexer::new(line);
    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let pipeline = match parser.parse() {
        Ok(pipeline) => pipeline,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };

    if let Some(mut pipeline) = pipeline {
        execute_pipeline(&mut pipeline);
    }
}
