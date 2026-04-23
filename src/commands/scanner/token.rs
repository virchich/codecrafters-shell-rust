#[derive(PartialEq)]
pub enum TokenType {
    Word, // command names, arguments, unquoted text

    Pipe,      // |
    Semicolon, // ;

    RedirectIn,           // <
    RedirectOut,          // >
    RedirectStdErr,       // 2>
    RedirectAppend,       // >>
    RedirectStdErrAppend, // 2>

    Ampersand, // &

    And, // &&

    Eof, // end of input
}

pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
}
