use std::fmt;

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
        LoxError { token: None, message, line }
    }
    pub fn parse_error(token: &Token, message: String) -> LoxError {
        LoxError { token: Some(token.clone()), message, line: token.line }
    }
    pub fn report(&self, loc: String) {
        println!("[{}:{}] Error: {}", self.line, loc, self.message);
    }
}
