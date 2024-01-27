use std::fmt;

use crate::{token::Token, token_type::TokenType};
#[derive(Debug)]
pub enum LoxResult {
    ParseError { token: Token, message: String },
    RuntimeError { token: Token, message: String },
    ScannerError { line: usize, message: String },
    SystemError { message: String },
    Break,
}

impl fmt::Display for LoxResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseError { token, message } => {
                write!(f, "Line: {}, Error: {}", token.line, message)
            }
            Self::RuntimeError { token, message } => {
                write!(f, "Line: {}, Error: {}", token.line, message)
            }

            Self::ScannerError { line, message } => write!(f, "Line: {}, Error: {}", line, message),
            Self::SystemError { message } => write!(f, "System error: {}", message),
            Self::Break => write!(f, ""),
        }
    }
}
impl LoxResult {
    pub fn error(line: usize, message: &str) -> LoxResult {
        let err = LoxResult::ScannerError {
            line,
            message: message.to_string(),
        };
        err.report();
        err
    }
    pub fn interp_error(token: &Token, message: &str) -> LoxResult {
        let err = LoxResult::RuntimeError {
            token: token.clone(),
            message: message.to_string(),
        };
        err.report();
        err
    }
    pub fn parse_error(token: &Token, message: &str) -> LoxResult {
        let err = LoxResult::ParseError {
            token: token.clone(),
            message: message.to_string(),
        };
        err.report();
        err
    }

    pub fn system_error(message: &str) -> LoxResult {
        let err = LoxResult::SystemError {
            message: message.to_string(),
        };
        err.report();
        err
    }

    pub fn report(&self) {
        match self {
            Self::ParseError { token, message } | Self::RuntimeError { token, message } => {
                if token.is(TokenType::Eof) {
                    eprintln!("Line: {} at end: {}", token.line, message)
                } else {
                    eprintln!("Line: {} at '{}' : {} ", token.line, token.lexeme, message);
                }
            }
            Self::ScannerError { line, message } => {
                eprintln!("Line: {}: Error: {}", line, message);
            }
            Self::SystemError { message } => {
                eprintln!("System Error: {}", message);
            }
            Self::Break => {}
        }
    }
}
