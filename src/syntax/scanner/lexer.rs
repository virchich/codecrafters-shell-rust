use crate::syntax::error::SyntaxError;
use crate::syntax::scanner::token::{Token, TokenType};

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    current: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),
            current: 0,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, SyntaxError> {
        while !self.is_at_end() {
            self.scan_token()?;
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
        });

        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), SyntaxError> {
        let char = self.advance();

        match char {
            ' ' | '\r' | '\t' | '\n' => {} // Ignore whitespace
            '&' => {
                if self.peek() == '&' {
                    self.add_token(TokenType::And, "&&".to_string());
                    self.advance(); // consume the second '&'
                } else {
                    self.add_token(TokenType::Ampersand, String::from(char))
                }
            }
            '|' => self.add_token(TokenType::Pipe, String::from(char)),
            ';' => self.add_token(TokenType::Semicolon, String::from(char)),
            '<' => self.add_token(TokenType::RedirectIn, String::from(char)),
            '>' => {
                if self.peek() == '>' {
                    self.advance();
                    self.add_token(TokenType::RedirectAppend, ">>".to_string());
                } else {
                    self.add_token(TokenType::RedirectOut, String::from(char));
                }
            }
            '1' => {
                if self.peek() == '>' && self.peek_next() == '>' {
                    self.advance();
                    self.advance();
                    self.add_token(TokenType::RedirectAppend, ">>".to_string());
                } else if self.peek() == '>' && self.peek_next() != '>' {
                    self.advance();
                    self.add_token(TokenType::RedirectOut, ">".to_string());
                } else {
                    self.scan_argument(char)?;
                }
            }
            '2' => {
                if self.peek() == '>' && self.peek_next() == '>' {
                    self.advance();
                    self.advance();
                    self.add_token(TokenType::RedirectStdErrAppend, "2>>".to_string());
                } else if self.peek() == '>' && self.peek_next() != '>' {
                    self.advance();
                    self.add_token(TokenType::RedirectStdErr, "2>".to_string());
                } else {
                    self.scan_argument(char)?;
                }
            }
            _ => self.scan_argument(char)?,
        }

        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: String) {
        self.tokens.push(Token { token_type, lexeme });
    }

    fn advance(&mut self) -> char {
        let char = self.source[self.current];
        self.current += 1;
        char
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() || self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn scan_argument(&mut self, first_char: char) -> Result<(), SyntaxError> {
        let mut buffer = String::new();

        // The first character was already consumed by scan_token.
        // If it's a quote, handle it; otherwise append it.
        match first_char {
            '\\' => {
                if !self.is_at_end() {
                    buffer.push(self.advance());
                }
            }
            '\'' => buffer.push_str(&self.scan_single_quote()?),
            '"' => buffer.push_str(&self.scan_double_quote()?),
            _ => buffer.push(first_char),
        }

        // Keep building the argument until we hit a separator.
        while !self.is_at_end() {
            match self.peek() {
                '\\' => {
                    self.advance(); // consume the backslash
                    if !self.is_at_end() {
                        buffer.push(self.advance()); // add the escaped character
                    }
                }
                '\'' => {
                    self.advance(); // consume the opening quote
                    buffer.push_str(&self.scan_single_quote()?);
                }
                '"' => {
                    self.advance(); // consume the opening quote
                    buffer.push_str(&self.scan_double_quote()?);
                }
                ' ' | '\r' | '\t' | '\n' | '|' | ';' | '<' | '>' => break,
                _ => {
                    buffer.push(self.advance());
                }
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::Word,
            lexeme: buffer,
        });

        Ok(())
    }

    fn scan_single_quote(&mut self) -> Result<String, SyntaxError> {
        let mut content = String::new();

        while !self.is_at_end() && self.peek() != '\'' {
            content.push(self.advance());
        }
        if self.is_at_end() {
            return Err(SyntaxError::Lexer("Unterminated single quote".to_string()));
        }

        // Consume the closing quote.
        self.advance();
        Ok(content)
    }

    fn scan_double_quote(&mut self) -> Result<String, SyntaxError> {
        let mut content = String::new();

        while !self.is_at_end() && self.peek() != '"' {
            if (self.peek() == '\\' && self.peek_next() == '\\')
                || (self.peek() == '\\' && self.peek_next() == '\"')
            {
                self.advance(); // consume the backslash
                if !self.is_at_end() {
                    content.push(self.advance()); // add the escaped character
                }
            } else {
                content.push(self.advance());
            }
        }
        if self.is_at_end() {
            return Err(SyntaxError::Lexer("Unterminated double quote".to_string()));
        }

        // Consume the closing quote.
        self.advance();
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::syntax::error::SyntaxError;
    use crate::syntax::scanner::token::TokenType;

    #[test]
    fn scans_words_quotes_and_operators() {
        let tokens = Lexer::new("echo \"two words\" 'three'|cat 2>>err &".to_string())
            .scan_tokens()
            .unwrap();

        let token_types: Vec<TokenType> = tokens.iter().map(|token| token.token_type).collect();
        let lexemes: Vec<&str> = tokens.iter().map(|token| token.lexeme.as_str()).collect();

        assert_eq!(
            token_types,
            vec![
                TokenType::Word,
                TokenType::Word,
                TokenType::Word,
                TokenType::Pipe,
                TokenType::Word,
                TokenType::RedirectStdErrAppend,
                TokenType::Word,
                TokenType::Ampersand,
                TokenType::Eof,
            ]
        );
        assert_eq!(
            lexemes,
            vec!["echo", "two words", "three", "|", "cat", "2>>", "err", "&", ""]
        );
    }

    #[test]
    fn keeps_escaped_characters_inside_arguments() {
        let tokens = Lexer::new("echo one\\ two \"a\\\\\\\"b\"".to_string())
            .scan_tokens()
            .unwrap();

        let lexemes: Vec<&str> = tokens.iter().map(|token| token.lexeme.as_str()).collect();
        assert_eq!(lexemes, vec!["echo", "one two", "a\\\"b", ""]);
    }

    #[test]
    fn recognizes_stdout_redirection_with_fd_prefix() {
        let tokens = Lexer::new("echo test 1>file >>append".to_string())
            .scan_tokens()
            .unwrap();

        let token_types: Vec<TokenType> = tokens.iter().map(|token| token.token_type).collect();
        assert_eq!(
            token_types,
            vec![
                TokenType::Word,
                TokenType::Word,
                TokenType::RedirectOut,
                TokenType::Word,
                TokenType::RedirectAppend,
                TokenType::Word,
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn returns_error_for_unterminated_single_quote() {
        let error = Lexer::new("echo 'missing".to_string())
            .scan_tokens()
            .unwrap_err();

        assert!(matches!(
            error,
            SyntaxError::Lexer(message) if message == "Unterminated single quote"
        ));
    }

    #[test]
    fn returns_error_for_unterminated_double_quote() {
        let error = Lexer::new("echo \"missing".to_string())
            .scan_tokens()
            .unwrap_err();

        assert!(matches!(
            error,
            SyntaxError::Lexer(message) if message == "Unterminated double quote"
        ));
    }
}
