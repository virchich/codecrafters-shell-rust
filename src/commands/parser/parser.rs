use crate::commands::command::Command;
use crate::commands::scanner::token::{Token, TokenType};
use crate::commands::statement::{Redirect, RedirectMode, RedirectStatement};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Option<RedirectStatement> {
        if self.tokens.is_empty() || self.tokens[0].token_type == TokenType::Eof {
            return None;
        }

        Some(self.redirect_statement())
    }

    fn redirect_statement(&mut self) -> RedirectStatement {
        let mut words: Vec<String> = Vec::new();
        let mut redirect_std_out: Option<Redirect> = None;
        let mut redirect_std_err: Option<Redirect> = None;

        while self.position < self.tokens.len()
            && self.tokens[self.position].token_type != TokenType::Eof
        {
            match self.tokens[self.position].token_type {
                TokenType::Word => {
                    words.push(self.tokens[self.position].lexeme.clone());
                    self.position += 1;
                }
                TokenType::RedirectOut | TokenType::RedirectAppend => {
                    let mode = if self.tokens[self.position].token_type == TokenType::RedirectAppend
                    {
                        RedirectMode::Append
                    } else {
                        RedirectMode::Overwrite
                    };
                    self.position += 1; // move past the redirect token

                    // Next token should be the file path
                    if self.position < self.tokens.len()
                        && self.tokens[self.position].token_type == TokenType::Word
                    {
                        redirect_std_out = Some(Redirect {
                            redirect_mode: mode,
                            file_location: self.tokens[self.position].lexeme.clone(),
                        });
                        self.position += 1;
                    }
                }
                TokenType::RedirectStdErr | TokenType::RedirectStdErrAppend => {
                    let mode = if self.tokens[self.position].token_type
                        == TokenType::RedirectStdErrAppend
                    {
                        RedirectMode::Append
                    } else {
                        RedirectMode::Overwrite
                    };
                    self.position += 1; // move past the redirect token

                    // Next token should be the file path
                    if self.position < self.tokens.len()
                        && self.tokens[self.position].token_type == TokenType::Word
                    {
                        redirect_std_err = Some(Redirect {
                            redirect_mode: mode,
                            file_location: self.tokens[self.position].lexeme.clone(),
                        });
                        self.position += 1;
                    }
                }
                _ => {
                    // Skip tokens we don't handle yet (Pipe, Semicolon, etc.)
                    self.position += 1;
                }
            }
        }

        RedirectStatement {
            command: Command {
                command: words.first().unwrap_or(&String::new()).clone(),
                arguments: words[1..].to_vec(),
            },
            redirect_std_out,
            redirect_std_err,
        }
    }
}
