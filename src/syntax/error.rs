use std::fmt;

pub enum SyntaxError {
    Lexer(String),
    Parser(String),
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxError::Lexer(message) => write!(formatter, "{}", message),
            SyntaxError::Parser(message) => write!(formatter, "Error parsing command: {}", message),
        }
    }
}
