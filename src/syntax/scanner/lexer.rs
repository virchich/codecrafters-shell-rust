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

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
        });

        self.tokens
    }

    fn scan_token(&mut self) {
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
                    self.scan_argument(char);
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
                    self.scan_argument(char);
                }
            }
            _ => self.scan_argument(char),
        }
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
        if self.is_at_end() {
            '\0'
        } else if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn scan_argument(&mut self, first_char: char) {
        let mut buffer = String::new();

        // The first character was already consumed by scan_token.
        // If it's a quote, handle it; otherwise append it.
        match first_char {
            '\\' => {
                if !self.is_at_end() {
                    buffer.push(self.advance());
                }
            }
            '\'' => buffer.push_str(&self.scan_single_quote()),
            '"' => buffer.push_str(&self.scan_double_quote()),
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
                    buffer.push_str(&self.scan_single_quote());
                }
                '"' => {
                    self.advance(); // consume the opening quote
                    buffer.push_str(&self.scan_double_quote());
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
    }

    fn scan_single_quote(&mut self) -> String {
        let mut content = String::new();

        while !self.is_at_end() && self.peek() != '\'' {
            content.push(self.advance());
        }
        if self.is_at_end() {
            eprintln!("Unterminated single quote");
            return content;
        }

        // Consume the closing quote.
        self.advance();
        content
    }

    fn scan_double_quote(&mut self) -> String {
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
            eprintln!("Unterminated double quote");
            return content;
        }

        // Consume the closing quote.
        self.advance();
        content
    }
}
