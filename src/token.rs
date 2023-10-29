use std::fmt;

use crate::token_type::TokenType;

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

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
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.ttype, self.lexeme, self.literal)
    }
}