use crate::commands::cmd::cd::cd;
use crate::commands::cmd::echo::echo;
use crate::commands::cmd::exec::exec;
use crate::commands::cmd::exit::exit;
use crate::commands::cmd::pwd::pwd;
use crate::commands::cmd::type_of::type_of;
use crate::commands::command::Command;
use crate::commands::scanner::lexer::Lexer;
use crate::commands::scanner::token::TokenType;
use std::io;
use std::io::Write;

pub fn read_command() -> Option<Command> {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();

    let scanner = Lexer::new(command);
    let parts = scanner.scan_tokens();

    if parts.is_empty() || parts[0].token_type == TokenType::Eof {
        return None;
    }

    Some(Command {
        command: parts[0].lexeme.clone(),
        // Exclude Eof token and command name
        arguments: parts[1..parts.len() - 1].iter().map(|token| token.lexeme.clone()).collect(),
    })
}

pub fn run_command(command: &Command) {
    match command.command.as_str() {
        "exit" => exit(command),
        "echo" => echo(command),
        "type" => type_of(command),
        "pwd" => pwd(command),
        "cd" => cd(command),
        _ => exec(command),
    }
}
