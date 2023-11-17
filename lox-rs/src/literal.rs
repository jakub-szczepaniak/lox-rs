use std::fmt::{self };


#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Identifier(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self,  f: &mut fmt::Formatter ) ->fmt::Result {
        let _ = match self {
             Literal::String(x) => write!(f, "\"{x}\""),
             Literal::Number(x) => write!(f, "{x}"),
             Literal::Boolean(x) => write!(f, "{x}"),
             Literal::Nil => write!(f, "nil"),
             Literal::Identifier(x) => write!(f, "var: {x}"),
        };
       Ok(())
    }
}