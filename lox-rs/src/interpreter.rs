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
                _ => Err(LoxError::interp_error(
                    &expr.operator,
                    "Unsupported operands",
                )),
            },
            TokenType::Slash => match (left, right) {
                (Literal::Number(_), Literal::Number(0.0)) => {
                    Err(LoxError::interp_error(&expr.operator, "Cannot divide by 0"))
                }
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x / y)),
                _ => Err(LoxError::interp_error(
                    &expr.operator,
                    "Unsupported operands",
                )),
            },
            TokenType::Star => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x * y)),
                _ => Err(LoxError::interp_error(
                    &expr.operator,
                    "Unsuported operands",
                )),
            },
            TokenType::Plus => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x + y)),
                (Literal::String(x), Literal::String(y)) => {
                    Ok(Literal::String(format!("{}{}", x, y)))
                }
                (Literal::String(x), Literal::Number(y)) => {
                    Ok(Literal::String(format!("{}{}", x, y)))
                }
                (Literal::Number(x), Literal::String(y)) => {
                    Ok(Literal::String(format!("{}{}", x, y)))
                }
                _ => Err(LoxError::interp_error(
                    &expr.operator,
                    "Unsupported operands",
                )),
            },
            TokenType::Greater => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean(x > y)),
                _ => Err(LoxError::interp_error(
                    &expr.operator,
                    "Unsuported operands",
                )),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean(x >= y)),
                _ => Err(LoxError::interp_error(
                    &expr.operator,
                    "Unsuported operands",
                )),
            },
            TokenType::Less => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean(x < y)),
                _ => Err(LoxError::interp_error(
                    &expr.operator,
                    "Unsuported operands",
                )),
            },
            TokenType::LessEqual => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean(x <= y)),
                _ => Err(LoxError::interp_error(
                    &expr.operator,
                    "Unsuported operands",
                )),
            },
            TokenType::BangEqual => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean(x != y)),
                (Literal::String(x), Literal::String(y)) => Ok(Literal::Boolean(!x.eq(&y))),
                (Literal::Boolean(x), Literal::Boolean(y)) => Ok(Literal::Boolean(x !=y )),
                _ => Ok(Literal::Boolean(true)),
            },
            TokenType::Equals => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean(x == y)),
                (Literal::String(x), Literal::String(y)) => Ok(Literal::Boolean(x.eq(&y))),
                (Literal::Boolean(x), Literal::Boolean(y)) => Ok(Literal::Boolean(x ==y )),
                (Literal::Nil, Literal::Nil) => Ok(Literal::Boolean(true)),
                _ => Ok(Literal::Boolean(false)),
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
    pub fn evaluate(&self, expr: &Expr) -> Result<Literal, LoxError> {
        expr.accept(self)
    }

    fn is_truthy(&self, literal: &Literal) -> bool {
        !matches!(literal, Literal::Nil | Literal::Boolean(false))
    }
}

#[cfg(test)]
mod tests {

    use rstest::*;
    use super::*;

    fn make_literal(literal: Literal) -> Box<Expr> {
        Box::new(Expr::Literal(ExprLiteral {
            value: Some(literal),
        }))
    }

    fn make_token_operator(ttype: TokenType, lexeme: &str) -> Token {
        Token::new(ttype, lexeme.to_string(), 0, None)
    }

    fn make_binary_expression(left: Box<Expr>, right: Box<Expr>, operator: Token) -> Expr {
        Expr::Binary(ExprBinary {
            left,
            operator,
            right,
        })
    }

    #[test]
    fn test_unary_minus_for_number() {
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
    fn test_unary_minus_for_string() {
        let interp = Interpreter {};
        let unary_minus_string = Expr::Unary(ExprUnary {
            operator: make_token_operator(TokenType::Minus, "-"),
            right: make_literal(Literal::String("some".to_string())),
        });

        let result = interp.evaluate(&unary_minus_string);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Nil));
    }

    #[rstest]
    #[case::bang_true (make_literal(Literal::Boolean(true)), false )]
    #[case::bang_false (make_literal(Literal::Boolean(false)), true )]
    #[case::bang_nil (make_literal(Literal::Nil), true )]
    fn test_unary_bang(#[case] input: Box<Expr>, #[case] expected: bool ) {
        let interp = Interpreter {};
        let unary_bang = Expr::Unary(ExprUnary {
            operator: make_token_operator(TokenType::Bang, "!"),
            right: input,
        });
        let result = interp.evaluate(&unary_bang);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(expected)));
    }

    #[test]
    fn test_binary_substraction() {
        let interp = Interpreter {};
        let binary_minus = make_binary_expression(
            make_literal(Literal::Number(13.0)),
            make_literal(Literal::Number(14.0)),
            make_token_operator(TokenType::Minus, "-"),
        );
        let result = interp.evaluate(&binary_minus);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Number(-1.0)));
    }

    #[test]
    fn test_binary_division() {
        let interp = Interpreter {};
        let binary_slash = make_binary_expression(
            make_literal(Literal::Number(10.0)),
            make_literal(Literal::Number(2.0)),
            make_token_operator(TokenType::Slash, "/"),
        );

        let result = interp.evaluate(&binary_slash);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Number(5.0)))
    }
    #[test]
    fn test_binary_division_by_zero() {
        let interp = Interpreter {};
        let binary_slash_by_zero = make_binary_expression(
            make_literal(Literal::Number(4.0)),
            make_literal(Literal::Number(0.0)),
            make_token_operator(TokenType::Slash, "/"),
        );

        let result = interp.evaluate(&binary_slash_by_zero);
        assert!(result.is_err());
    }
    #[test]
    fn test_binary_multiplication() {
        let interp = Interpreter {};
        let binary_star = make_binary_expression(
            make_literal(Literal::Number(2.0)),
            make_literal(Literal::Number(2.0)),
            make_token_operator(TokenType::Star, "*"),
        );
        let result = interp.evaluate(&binary_star);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Number(4.0)));
    }
    #[test]
    fn test_binary_addition_numeric() {
        let interp = Interpreter {};
        let binary_plus = make_binary_expression(
            make_literal(Literal::Number(2.0)),
            make_literal(Literal::Number(2.0)),
            make_token_operator(TokenType::Plus, "+"),
        );
        let result = interp.evaluate(&binary_plus);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Number(4.0)));
    }
    #[test]
    fn test_binary_concatenation_string() {
        let interp = Interpreter {};
        let binary_concat = make_binary_expression(
            make_literal(Literal::String("Hello".to_string())),
            make_literal(Literal::String(" world!".to_string())),
            make_token_operator(TokenType::Plus, "+"),
        );

        let result = interp.evaluate(&binary_concat);

        assert!(result.is_ok());
        assert_eq!(
            result.ok(),
            Some(Literal::String("Hello world!".to_string()))
        );
    }

    #[test]
    fn test_binary_concatenation_string_number() {
        let interp = Interpreter {};
        let binary_concat = make_binary_expression(
            make_literal(Literal::String("123".to_string())),
            make_literal(Literal::Number(4.0)),
            make_token_operator(TokenType::Plus, "+"),
        );

        let result = interp.evaluate(&binary_concat);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::String("1234".to_string())));
    }
    #[test]
    fn test_binary_concatenation_number_string() {
        let interp = Interpreter {};
        let binary_concat = make_binary_expression(
            make_literal(Literal::Number(4.0)),
            make_literal(Literal::String("123".to_string())),
            make_token_operator(TokenType::Plus, "+"),
        );

        let result = interp.evaluate(&binary_concat);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::String("4123".to_string())));
    }
    #[test]
    fn test_binary_greater_than() {
        let interp = Interpreter {};
        let binary_concat = make_binary_expression(
            make_literal(Literal::Number(6.0)),
            make_literal(Literal::Number(5.0)),
            make_token_operator(TokenType::Greater, ">"),
        );

        let result = interp.evaluate(&binary_concat);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
    }
    #[test]
    fn test_binary_greater_equal_than_is_greater() {
        let interp = Interpreter {};
        let binary_concat = make_binary_expression(
            make_literal(Literal::Number(6.0)),
            make_literal(Literal::Number(5.0)),
            make_token_operator(TokenType::GreaterEqual, ">="),
        );

        let result = interp.evaluate(&binary_concat);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
    }
    #[test]
    fn test_binary_greater_equal_than_is_equal() {
        let interp = Interpreter {};
        let binary_concat = make_binary_expression(
            make_literal(Literal::Number(6.0)),
            make_literal(Literal::Number(6.0)),
            make_token_operator(TokenType::GreaterEqual, ">="),
        );

        let result = interp.evaluate(&binary_concat);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
    }
    #[test]
    fn test_binary_less_than() {
        let interp = Interpreter {};
        let binary_less = make_binary_expression(
            make_literal(Literal::Number(5.0)),
            make_literal(Literal::Number(6.0)),
            make_token_operator(TokenType::Less, "<"),
        );

        let result = interp.evaluate(&binary_less);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
    }
    #[test]
    fn test_binary_less_than_equal_is_less() {
        let interp = Interpreter {};
        let binary_less = make_binary_expression(
            make_literal(Literal::Number(5.0)),
            make_literal(Literal::Number(6.0)),
            make_token_operator(TokenType::LessEqual, "<="),
        );

        let result = interp.evaluate(&binary_less);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
    }
     #[test]
    fn test_binary_less_than_equal_is_equal() {
        let interp = Interpreter {};
        let binary_less = make_binary_expression(
            make_literal(Literal::Number(6.0)),
            make_literal(Literal::Number(6.0)),
            make_token_operator(TokenType::LessEqual, "<="),
        );

        let result = interp.evaluate(&binary_less);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
    }
     #[rstest]
     #[case::two_numbers (make_literal(Literal::Number(6.0)), make_literal(Literal::Number(4.0)))]
     #[case::two_strings (make_literal(Literal::String("abc".to_string())), make_literal(Literal::String("bta".to_string())))]
     #[case::two_booleans (make_literal(Literal::Boolean(true)), make_literal(Literal::Boolean(false)))]
     #[case::mixed_types (make_literal(Literal::Number(3.0)), make_literal(Literal::Boolean(false)))]
     #[case::mixed_types (make_literal(Literal::Nil), make_literal(Literal::Boolean(false)))]
     #[case::mixed_types (make_literal(Literal::Nil), make_literal(Literal::Number(2.3)))]
     #[case::mixed_types (make_literal(Literal::String("false".to_string())), make_literal(Literal::Boolean(false)))]
    fn test_binary_bang_equal_inputs(#[case] left: Box<Expr>, #[case] right: Box<Expr>) {
        let interp = Interpreter {};
        let binary_bang_equal = make_binary_expression(
            left, right, make_token_operator(TokenType::BangEqual, "!="),
        );

        let result = interp.evaluate(&binary_bang_equal);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
    }
    #[rstest]
    #[case::two_numbers (make_literal(Literal::Number(6.0)), make_literal(Literal::Number(6.0)))]
    #[case::two_strings (make_literal(Literal::String("abc".to_string())), make_literal(Literal::String("abc".to_string())))]
    #[case::two_booleans (make_literal(Literal::Boolean(true)), make_literal(Literal::Boolean(true)))]
    #[case::two_nulls (make_literal(Literal::Nil), make_literal(Literal::Nil))]
    fn test_binary_equal_equal(#[case] left: Box<Expr>, #[case] right: Box<Expr>) {
        let interp = Interpreter {};
        let binary_equal = make_binary_expression(
            left, right, make_token_operator(TokenType::Equals, "=="),
        );

        let result = interp.evaluate(&binary_equal);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
 
    }     
    

}
