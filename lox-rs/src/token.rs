use std::fmt;

use crate::token_type::TokenType;

#[derive(Debug)]
pub enum Literal {
    String(String),
    Identifier(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    line: usize,
    literal: Option<Literal>,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, line: usize, literal: Option<Literal>) -> Token {
        Token {
            ttype,
            lexeme,
            line,
            literal,
        }
    }
    pub fn eof(line: usize) -> Token {
        Token {
            ttype: TokenType::Eof,
            lexeme: "".to_string(),
            line,
            literal: None,       
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:}]{:?} {} {:?}", self.line, self.ttype, self.lexeme, self.literal)
    }
}