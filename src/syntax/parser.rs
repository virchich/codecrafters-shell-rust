use crate::syntax::command_invocation::CommandInvocation;
use crate::syntax::error::SyntaxError;
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

    pub fn parse(&mut self) -> Result<Option<Pipeline>, SyntaxError> {
        if self.tokens.is_empty() || self.tokens[0].token_type == TokenType::Eof {
            return Ok(None);
        }

        self.pipeline().map(Some)
    }

    fn pipeline(&mut self) -> Result<Pipeline, SyntaxError> {
        let mut commands: Vec<CommandInvocation> = Vec::new();
        let mut is_background = false;

        while self.position < self.tokens.len()
            && self.tokens[self.position].token_type != TokenType::Eof
        {
            commands.push(self.command_invocation()?);

            match self.peek_token_type() {
                TokenType::Pipe => {
                    self.advance();
                    if self.peek_token_type() == TokenType::Eof {
                        return Err(parser_error("Syntax error: expected command after '|'"));
                    }
                }
                TokenType::Ampersand => {
                    self.advance();
                    is_background = true;

                    if self.peek_token_type() != TokenType::Eof {
                        return Err(parser_error(
                            "Syntax error: '&' must be at the end of the command",
                        ));
                    }
                }
                TokenType::Eof => {}
                _ => break,
            }
        }

        Ok(Pipeline {
            commands,
            is_background,
        })
    }

    fn command_invocation(&mut self) -> Result<CommandInvocation, SyntaxError> {
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
                    let redirection_token = self.tokens[self.position].lexeme.clone();
                    let mode = if self.tokens[self.position].token_type == TokenType::RedirectAppend
                    {
                        RedirectionMode::Append
                    } else {
                        RedirectionMode::Overwrite
                    };
                    self.position += 1;

                    let file_path = self.consume_redirection_target(&redirection_token)?;
                    stdout_redirection = Some(Redirection { mode, file_path });
                }
                TokenType::RedirectStdErr | TokenType::RedirectStdErrAppend => {
                    let redirection_token = self.tokens[self.position].lexeme.clone();
                    let mode = if self.tokens[self.position].token_type
                        == TokenType::RedirectStdErrAppend
                    {
                        RedirectionMode::Append
                    } else {
                        RedirectionMode::Overwrite
                    };
                    self.position += 1;

                    let file_path = self.consume_redirection_target(&redirection_token)?;
                    stderr_redirection = Some(Redirection { mode, file_path });
                }
                TokenType::And => {
                    return Err(parser_error("Syntax error: unsupported operator '&&'"));
                }
                TokenType::Semicolon => {
                    return Err(parser_error("Syntax error: unsupported operator ';'"));
                }
                TokenType::RedirectIn => {
                    return Err(parser_error("Syntax error: unsupported redirection '<'"));
                }
                token_type => {
                    return Err(parser_error(format!(
                        "Syntax error: unexpected token '{}'",
                        display_token(token_type)
                    )));
                }
            }
        }

        if words.is_empty() {
            return Err(parser_error("Syntax error: expected command"));
        }

        Ok(CommandInvocation {
            name: words.first().cloned().unwrap_or_default(),
            arguments: words[1..].to_vec(),
            stdout_redirection,
            stderr_redirection,
        })
    }

    fn consume_redirection_target(
        &mut self,
        redirection_token: &str,
    ) -> Result<String, SyntaxError> {
        if self.peek_token_type() != TokenType::Word {
            return Err(parser_error(format!(
                "Syntax error: expected file after '{}'",
                redirection_token
            )));
        }

        let file_path = self.tokens[self.position].lexeme.clone();
        self.advance();

        Ok(file_path)
    }

    fn peek_token_type(&self) -> TokenType {
        self.tokens
            .get(self.position)
            .map(|token| token.token_type)
            .unwrap_or(TokenType::Eof)
    }

    fn advance(&mut self) {
        self.position += 1;
    }
}

fn parser_error(message: impl Into<String>) -> SyntaxError {
    SyntaxError::Parser(message.into())
}

fn display_token(token_type: TokenType) -> &'static str {
    match token_type {
        TokenType::Word => "word",
        TokenType::Pipe => "|",
        TokenType::Semicolon => ";",
        TokenType::RedirectIn => "<",
        TokenType::RedirectOut => ">",
        TokenType::RedirectStdErr => "2>",
        TokenType::RedirectAppend => ">>",
        TokenType::RedirectStdErrAppend => "2>>",
        TokenType::Ampersand => "&",
        TokenType::And => "&&",
        TokenType::Eof => "end of input",
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::syntax::error::SyntaxError;
    use crate::syntax::redirection::RedirectionMode;
    use crate::syntax::scanner::lexer::Lexer;

    fn parse(source: &str) -> Result<Option<crate::syntax::pipeline::Pipeline>, SyntaxError> {
        let tokens = Lexer::new(source.to_string()).scan_tokens().unwrap();
        Parser::new(tokens).parse()
    }

    #[test]
    fn returns_none_for_empty_input() {
        let pipeline = parse("").unwrap();
        assert!(pipeline.is_none());
    }

    #[test]
    fn parses_pipeline_background_and_redirections() {
        let pipeline = parse("echo hello > out | cat 2>> err &")
            .unwrap()
            .unwrap();

        assert_eq!(pipeline.commands.len(), 2);
        assert!(pipeline.is_background);

        let first = &pipeline.commands[0];
        assert_eq!(first.name, "echo");
        assert_eq!(first.arguments, vec!["hello"]);
        assert!(matches!(
            first.stdout_redirection.as_ref().map(|redirection| &redirection.mode),
            Some(RedirectionMode::Overwrite)
        ));
        assert_eq!(
            first.stdout_redirection.as_ref().unwrap().file_path,
            "out"
        );

        let second = &pipeline.commands[1];
        assert_eq!(second.name, "cat");
        assert!(second.stdout_redirection.is_none());
        assert!(matches!(
            second.stderr_redirection.as_ref().map(|redirection| &redirection.mode),
            Some(RedirectionMode::Append)
        ));
        assert_eq!(
            second.stderr_redirection.as_ref().unwrap().file_path,
            "err"
        );
    }

    #[test]
    fn errors_when_pipe_has_no_command_after_it() {
        let error = parse("echo hi |").unwrap_err();
        assert_eq!(
            error.to_string(),
            "Error parsing command: Syntax error: expected command after '|'"
        );
    }

    #[test]
    fn errors_when_ampersand_is_not_last() {
        let error = parse("echo hi & pwd").unwrap_err();
        assert_eq!(
            error.to_string(),
            "Error parsing command: Syntax error: '&' must be at the end of the command"
        );
    }

    #[test]
    fn errors_for_unsupported_and_operator() {
        let error = parse("echo hi && pwd").unwrap_err();
        assert_eq!(
            error.to_string(),
            "Error parsing command: Syntax error: unsupported operator '&&'"
        );
    }

    #[test]
    fn errors_when_redirection_target_is_missing() {
        let error = parse("echo >").unwrap_err();
        assert_eq!(
            error.to_string(),
            "Error parsing command: Syntax error: expected file after '>'"
        );
    }

    #[test]
    fn errors_when_command_is_missing() {
        let error = parse("> out").unwrap_err();
        assert_eq!(
            error.to_string(),
            "Error parsing command: Syntax error: expected command"
        );
    }
}
