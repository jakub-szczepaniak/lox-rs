use crate::error::LoxError;
use crate::expr::*;
use crate::literal::*;
use crate::token::Token;
use crate::token_type::*;

pub struct Interpreter {}

impl ExprVisitor<Literal> for Interpreter {
    fn visit_binary_expr(&self, expr: &ExprBinary) -> Result<Literal, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type() {
            TokenType::Minus => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x - y)),
                _ => {
                    todo!("not yet implemented")
                }
            },
            TokenType::Slash => match (left, right) {
                (Literal::Number(_), Literal::Number(0.0)) => {
                    Err(LoxError::error(0, "Cannot divide by 0"))
                }
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x / y)),
                _ => {
                    todo!("Not implemented")
                }
            },
            _ => {
                todo!("not implemented")
            }
        }
    }
    fn visit_grouping_expr(&self, expr: &ExprGrouping) -> Result<Literal, LoxError> {
        Ok(self.evaluate(&expr.expression).unwrap())
    }

    fn visit_literal_expr(&self, expr: &ExprLiteral) -> Result<Literal, LoxError> {
        if let Some(value) = &expr.value {
            Ok(value.clone())
        } else {
            Err(LoxError::error(0, "Cannot evaluate expression"))
        }
    }

    fn visit_unary_expr(&self, expr: &ExprUnary) -> Result<Literal, LoxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type() {
            TokenType::Minus => match right {
                Literal::Number(x) => Ok(Literal::Number(-x)),
                _ => Ok(Literal::Nil),
            },
            TokenType::Bang => Ok(Literal::Boolean(!self.is_truthy(&right))),
            _ => Err(LoxError {
                token: Some(expr.operator.clone()),
                message: "Mismatch type to operator".to_string(),
                line: expr.operator.line,
            }),
        }
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Literal, LoxError> {
        expr.accept(self)
    }

    fn is_truthy(&self, literal: &Literal) -> bool {
        !matches!(literal, Literal::Nil | Literal::Boolean(false))
    }
}

#[cfg(test)]
mod tests {

    use std::{io::LineWriter, result};

    use super::*;

    fn make_literal(literal: Literal) -> Box<Expr> {
        Box::new(Expr::Literal(ExprLiteral {
            value: Some(literal),
        }))
    }

    fn make_token_operator(ttype: TokenType, lexeme: &str) -> Token {
        Token::new(ttype, lexeme.to_string(), 0, None)
    }

    #[test]
    fn test_the_unary_minus_for_number() {
        let interp = Interpreter {};
        let unary_minus = Expr::Unary(ExprUnary {
            operator: make_token_operator(TokenType::Minus, "-"),
            right: make_literal(Literal::Number(42.0)),
        });
        let result = interp.evaluate(&unary_minus);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Number(-42.0)))
    }

    #[test]
    fn test_the_unary_minus_for_string() {
        let interp = Interpreter {};
        let unary_minus_string = Expr::Unary(ExprUnary {
            operator: make_token_operator(TokenType::Minus, "."),
            right: make_literal(Literal::String("some".to_string())),
        });

        let result = interp.evaluate(&unary_minus_string);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Nil));
    }

    #[test]
    fn test_unary_bang_true_result_false() {
        let interp = Interpreter {};
        let unary_bang_true = Expr::Unary(ExprUnary {
            operator: make_token_operator(TokenType::Bang, "!"),
            right: make_literal(Literal::Boolean(true)),
        });
        let result = interp.evaluate(&unary_bang_true);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(false)));
    }

    #[test]
    fn test_unary_bang_false_result_true() {
        let interp = Interpreter {};
        let unary_bang_false = Expr::Unary(ExprUnary {
            operator: make_token_operator(TokenType::Bang, "!"),
            right: make_literal(Literal::Boolean(false)),
        });

        let result = interp.evaluate(&unary_bang_false);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
    }

    #[test]
    fn test_unary_bang_nil_result_true() {
        let interp = Interpreter {};
        let unary_bang_nil = Expr::Unary(ExprUnary {
            operator: make_token_operator(TokenType::Bang, "!"),
            right: make_literal(Literal::Nil),
        });

        let result = interp.evaluate(&unary_bang_nil);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
    }

    #[test]
    fn test_binary_substraction() {
        let interp = Interpreter {};
        let binary_minus = Expr::Binary(ExprBinary {
            left: make_literal(Literal::Number(13.0)),
            operator: make_token_operator(TokenType::Minus, "-"),
            right: make_literal(Literal::Number(14.0)),
        });
        let result = interp.evaluate(&binary_minus);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Number(-1.0)));
    }

    #[test]
    fn test_binary_division() {
        let interp = Interpreter {};
        let binary_slash = Expr::Binary(ExprBinary {
            left: make_literal(Literal::Number(10.0)),
            operator: make_token_operator(TokenType::Slash, "/"),
            right: make_literal(Literal::Number(2.0)),
        });

        let result = interp.evaluate(&binary_slash);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Number(5.0)))
    }
    #[test]
    fn test_binary_division_by_zero() {
        let interp = Interpreter {};
        let binary_slash_by_zero = Expr::Binary(ExprBinary {
            left: make_literal(Literal::Number(42.0)),
            operator: make_token_operator(TokenType::Slash, "/"),
            right: make_literal(Literal::Number(0.0)),
        });

        let result = interp.evaluate(&binary_slash_by_zero);
        assert!(result.is_err());
    }
}
