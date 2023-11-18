use std::fmt;

use crate::{token::Token, token_type::TokenType};

#[derive(Debug)]
pub struct LoxError {
    pub token: Option<Token>,
    pub message: String,
    pub line: usize,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line: {}, Error: {}", self.line, self.message)
    }
}
impl LoxError {
    pub fn error(line: usize, message: String) -> LoxError {
        let err = LoxError {
            token: None,
            message,
            line,
        };
        err.report("".to_string());
        err
    }
    pub fn parse_error(token: &Token, message: String) -> LoxError {
        let err = LoxError {
            token: Some(token.clone()),
            message,
            line: token.line,
        };
        err.report("".to_string());
        err
    }
    pub fn report(&self, loc: String) {
        if let Some(token) = &self.token {
            if token.is(TokenType::Eof) {
                eprintln!("{} at end: {}", token.line, self.message)
            } else {
                eprintln!("{} at '{}' : {} ", token.line, token.lexeme, self.message)
            }
        } else {
            eprintln!("[line {}:{}] Error:{}", self.line, loc, self.message)
        }
    }
}
