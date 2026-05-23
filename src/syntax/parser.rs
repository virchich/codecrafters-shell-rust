use crate::syntax::command_invocation::CommandInvocation;
use crate::syntax::pipeline::Pipeline;
use crate::syntax::redirection::{Redirection, RedirectionMode};
use crate::syntax::scanner::token::{Token, TokenType};

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

    pub fn parse(&mut self) -> Option<Pipeline> {
        if self.tokens.is_empty() || self.tokens[0].token_type == TokenType::Eof {
            return None;
        }

        match self.pipeline() {
            Ok(pipeline) => Some(pipeline),
            Err(error) => {
                eprintln!("Error parsing command: {}", error);

                None
            }
        }
    }

    fn pipeline(&mut self) -> Result<Pipeline, String> {
        let mut commands: Vec<CommandInvocation> = Vec::new();
        let mut is_background = false;

        while self.position < self.tokens.len()
            && self.tokens[self.position].token_type != TokenType::Eof
        {
            commands.push(self.command_invocation());

            // If the next token is a pipe, skip it and continue parsing the next command.
            if self.position < self.tokens.len()
                && self.tokens[self.position].token_type == TokenType::Pipe
            {
                self.position += 1;
            } else if self.position < self.tokens.len()
                && self.tokens[self.position].token_type == TokenType::Ampersand
            {
                self.position += 1;
                is_background = true;

                // A background marker is only valid at the end of a pipeline.
                if self.tokens[self.position].token_type != TokenType::Eof {
                    return Err("Syntax error: '&' must be at the end of the command".to_string());
                }
            } else {
                break;
            }
        }

        Ok(Pipeline {
            commands,
            is_background,
        })
    }

    fn command_invocation(&mut self) -> CommandInvocation {
        let mut words: Vec<String> = Vec::new();
        let mut stdout_redirection: Option<Redirection> = None;
        let mut stderr_redirection: Option<Redirection> = None;

        while self.position < self.tokens.len()
            && self.tokens[self.position].token_type != TokenType::Eof
            && self.tokens[self.position].token_type != TokenType::Pipe
            && self.tokens[self.position].token_type != TokenType::Ampersand
        {
            match self.tokens[self.position].token_type {
                TokenType::Word => {
                    words.push(self.tokens[self.position].lexeme.clone());
                    self.position += 1;
                }
                TokenType::RedirectOut | TokenType::RedirectAppend => {
                    let mode = if self.tokens[self.position].token_type == TokenType::RedirectAppend
                    {
                        RedirectionMode::Append
                    } else {
                        RedirectionMode::Overwrite
                    };
                    self.position += 1;

                    // Next token should be the file path.
                    if self.position < self.tokens.len()
                        && self.tokens[self.position].token_type == TokenType::Word
                    {
                        stdout_redirection = Some(Redirection {
                            mode,
                            file_path: self.tokens[self.position].lexeme.clone(),
                        });
                        self.position += 1;
                    }
                }
                TokenType::RedirectStdErr | TokenType::RedirectStdErrAppend => {
                    let mode = if self.tokens[self.position].token_type
                        == TokenType::RedirectStdErrAppend
                    {
                        RedirectionMode::Append
                    } else {
                        RedirectionMode::Overwrite
                    };
                    self.position += 1;

                    // Next token should be the file path.
                    if self.position < self.tokens.len()
                        && self.tokens[self.position].token_type == TokenType::Word
                    {
                        stderr_redirection = Some(Redirection {
                            mode,
                            file_path: self.tokens[self.position].lexeme.clone(),
                        });
                        self.position += 1;
                    }
                }
                _ => {
                    // Skip tokens we don't handle yet.
                    self.position += 1;
                }
            }
        }

        CommandInvocation {
            name: words.first().cloned().unwrap_or_default(),
            arguments: words[1..].to_vec(),
            stdout_redirection,
            stderr_redirection,
        }
    }
}
