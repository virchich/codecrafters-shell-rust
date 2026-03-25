use crate::commands::cmd::cd::cd;
use crate::commands::cmd::echo::echo;
use crate::commands::cmd::exec::exec;
use crate::commands::cmd::exit::exit;
use crate::commands::cmd::pwd::pwd;
use crate::commands::cmd::type_of::type_of;
use crate::commands::parser::parser::Parser;
use crate::commands::scanner::lexer::Lexer;
use crate::commands::statement::{Redirect, RedirectMode, RedirectStatement};
use std::fs::File;
use std::io;
use std::io::Write;

pub fn read_line() -> Option<RedirectStatement> {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();

    let scanner = Lexer::new(command);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);

    parser.parse()
}

fn open_redirect(redirect: &Redirect) -> File {
    match redirect.redirect_mode {
        RedirectMode::Append => File::options()
            .create(true)
            .append(true)
            .open(&redirect.file_location)
            .unwrap(),
        RedirectMode::Overwrite => File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&redirect.file_location)
            .unwrap(),
    }
}

pub fn run_statement(statement: &RedirectStatement) {
    let mut stdout_writer: Box<dyn Write> = if let Some(redirect) = &statement.redirect_std_out {
        Box::new(open_redirect(redirect))
    } else {
        Box::new(io::stdout())
    };

    let mut stderr_writer: Box<dyn Write> = if let Some(redirect) = &statement.redirect_std_err {
        Box::new(open_redirect(redirect))
    } else {
        Box::new(io::stderr())
    };

    match statement.command.command.as_str() {
        "exit" => exit(&statement.command, &mut *stderr_writer),
        "echo" => echo(&statement.command, &mut *stdout_writer),
        "type" => type_of(&statement.command, &mut *stdout_writer, &mut *stderr_writer),
        "pwd" => pwd(&statement.command, &mut *stdout_writer, &mut *stderr_writer),
        "cd" => cd(&statement.command, &mut *stderr_writer),
        _ => exec(&statement),
    }
}
