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
    pub fn error(line: usize, message: &str) -> LoxError {
        let err = LoxError {
            token: None,
            message: message.to_string(),
            line,
        };
        err.report("");
        err
    }
    pub fn interp_error(token: &Token, message: &str) -> LoxError {
        let err = LoxError {
            token: Some(token.clone()), 
            message: message.to_string(),
            line: token.line
        };
        err.report("");
        err
    }
    pub fn parse_error(token: &Token, message: &str) -> LoxError {
        let err = LoxError {
            token: Some(token.clone()),
            message: message.to_string(),
            line: token.line,
        };
        err.report("");
        err
    }
    pub fn report(&self, loc: &str) {
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
