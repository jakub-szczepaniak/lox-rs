use std::{fmt, thread::yield_now};

use crate::token::Token;
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
        let err = LoxError { token: None, message, line };
        err.report("".to_string());
        err
    }
    pub fn parse_error(token: &Token, message: String) -> LoxError {
        let err = LoxError { token: Some(token.clone()), message, line: token.line };
        err.report("".to_string());
        err
        
    }
    pub fn report(&self, loc: String) {
        println!("[{}:{}] Error: {}", self.line, loc, self.message);
    }
}
