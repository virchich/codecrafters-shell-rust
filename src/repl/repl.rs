use crate::commands::core::run_statement;
use crate::commands::parser::parser::Parser;
use crate::commands::scanner::lexer::Lexer;
use crate::repl::repl_helper::ReplHelper;
use rustyline::config::BellStyle;
use rustyline::{CompletionType, Config, Editor};

pub fn repl() {
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .bell_style(BellStyle::Audible)
        .max_history_size(100).unwrap()
        .build();

    let mut editor: Editor<ReplHelper, _> = Editor::with_config(config).unwrap();
    editor.set_helper(Some(ReplHelper::new()));

    loop {
        let line = editor.readline("$ ");

        match line {
            Ok(line) => {
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
