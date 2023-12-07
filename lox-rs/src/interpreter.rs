use crate::environment::*;
use crate::error::LoxResult;
use crate::expr::*;
use crate::literal::*;
use crate::stmt::*;
use crate::token_type::*;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Interpreter {
    environment: RefCell<Rc<RefCell<Environment>>>,
    in_loop: RefCell<usize>,
}

impl StmtVisitor<()> for Interpreter {
    fn visit_break_stmt(&self, expr: &StmtBreak) -> Result<(), LoxResult> {
        if *self.in_loop.borrow() == 0 {
            Err(LoxResult::interp_error(
                &expr.token,
                "Break statement outside of the loop!",
            ))
        } else {
            Err(LoxResult::Break)
        }
    }

    fn visit_block_stmt(&self, stmt_block: &StmtBlock) -> Result<(), LoxResult> {
        let env = Environment::nested_in(self.environment.borrow().clone());
        self.execute_block(&stmt_block.statements, env)
    }
    fn visit_expression_stmt(&self, expr: &StmtExpression) -> Result<(), LoxResult> {
        self.evaluate(&expr.expression)?;
        Ok(())
    }
    fn visit_print_stmt(&self, expr: &StmtPrint) -> Result<(), LoxResult> {
        let value = self.evaluate(&expr.expression)?;
        println!("{}", value);
        Ok(())
    }
    fn visit_var_stmt(&self, expr: &StmtVar) -> Result<(), LoxResult> {
        let value = if let Some(initializer) = &expr.initializer {
            self.evaluate(initializer)?
        } else {
            Literal::Nil
        };
        self.environment
            .borrow()
            .borrow_mut()
            .define(expr.name.as_string(), value);
        Ok(())
    }
    fn visit_if_stmt(&self, stmt: &StmtIf) -> Result<(), LoxResult> {
        if self.is_truthy(&self.evaluate(&stmt.condition)?) {
            self.execute(&stmt.then_branch)
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }

    fn visit_while_stmt(&self, stmt: &StmtWhile) -> Result<(), LoxResult> {
        *self.in_loop.borrow_mut() += 1;
        while self.is_truthy(&self.evaluate(&stmt.condition)?) {
            match self.execute(&stmt.body) {
                Err(LoxResult::Break) => break,
                Err(e) => return Err(e),
                Ok(_) => {}
            }
        }
        *self.in_loop.borrow_mut() -= 1;
        Ok(())
    }
}

impl ExprVisitor<Literal> for Interpreter {
    fn visit_logical_expr(&self, expr: &ExprLogical) -> Result<Literal, LoxResult> {
        let left = self.evaluate(&expr.left)?;

        if expr.operator.is(TokenType::Or) {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else if !self.is_truthy(&left) {
            return Ok(left);
        }

        self.evaluate(&expr.right)
    }

    fn visit_assign_expr(&self, expr: &ExprAssign) -> Result<Literal, LoxResult> {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow()
            .borrow_mut()
            .assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn visit_variable_expr(&self, expr: &ExprVariable) -> Result<Literal, LoxResult> {
        self.environment.borrow().borrow().get(&expr.name)
    }

    fn visit_binary_expr(&self, expr: &ExprBinary) -> Result<Literal, LoxResult> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type() {
            TokenType::Minus => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x - y)),
                _ => Err(LoxResult::interp_error(
                    &expr.operator,
                    "Unsupported operands",
                )),
            },
            TokenType::Slash => match (left, right) {
                (Literal::Number(_), Literal::Number(0.0)) => Err(LoxResult::interp_error(
                    &expr.operator,
                    "Cannot divide by 0",
                )),
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x / y)),
                _ => Err(LoxResult::interp_error(
                    &expr.operator,
                    "Unsupported operands",
                )),
            },
            TokenType::Star => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x * y)),
                _ => Err(LoxResult::interp_error(
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
                _ => Err(LoxResult::interp_error(
                    &expr.operator,
                    "Unsupported operands",
                )),
            },
            TokenType::Greater => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean(x > y)),
                _ => Err(LoxResult::interp_error(
                    &expr.operator,
                    "Unsuported operands",
                )),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean(x >= y)),
                _ => Err(LoxResult::interp_error(
                    &expr.operator,
                    "Unsuported operands",
                )),
            },
            TokenType::Less => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean(x < y)),
                _ => Err(LoxResult::interp_error(
                    &expr.operator,
                    "Unsuported operands",
                )),
            },
            TokenType::LessEqual => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean(x <= y)),
                _ => Err(LoxResult::interp_error(
                    &expr.operator,
                    "Unsuported operands",
                )),
            },
            TokenType::BangEqual => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean(x != y)),
                (Literal::String(x), Literal::String(y)) => Ok(Literal::Boolean(!x.eq(&y))),
                (Literal::Boolean(x), Literal::Boolean(y)) => Ok(Literal::Boolean(x != y)),
                _ => Ok(Literal::Boolean(true)),
            },
            TokenType::Equals => match (left, right) {
                (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Boolean(x == y)),
                (Literal::String(x), Literal::String(y)) => Ok(Literal::Boolean(x.eq(&y))),
                (Literal::Boolean(x), Literal::Boolean(y)) => Ok(Literal::Boolean(x == y)),
                (Literal::Nil, Literal::Nil) => Ok(Literal::Boolean(true)),
                _ => Ok(Literal::Boolean(false)),
            },

            _ => {
                todo!("not implemented")
            }
        }
    }
    fn visit_grouping_expr(&self, expr: &ExprGrouping) -> Result<Literal, LoxResult> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &ExprLiteral) -> Result<Literal, LoxResult> {
        if let Some(value) = &expr.value {
            Ok(value.clone())
        } else {
            Err(LoxResult::error(0, "Cannot evaluate expression"))
        }
    }

    fn visit_unary_expr(&self, expr: &ExprUnary) -> Result<Literal, LoxResult> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type() {
            TokenType::Minus => match right {
                Literal::Number(x) => Ok(Literal::Number(-x)),
                _ => Ok(Literal::Nil),
            },
            TokenType::Bang => Ok(Literal::Boolean(!self.is_truthy(&right))),
            _ => Err(LoxResult::RuntimeError {
                token: expr.operator.clone(),
                message: "Mismatch type to operator".to_string(),
            }),
        }
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: RefCell::new(Rc::new(RefCell::new(Environment::new()))),
            in_loop: RefCell::new(0),
        }
    }

    fn execute_block(&self, statements: &[Stmt], env: Environment) -> Result<(), LoxResult> {
        let previous = self.environment.replace(Rc::new(RefCell::new(env)));

        let result = statements
            .iter()
            .try_for_each(|statement| self.execute(statement));

        self.environment.replace(previous);
        result
    }

    fn evaluate(&self, expr: &Expr) -> Result<Literal, LoxResult> {
        expr.accept(self)
    }
    fn execute(&self, stmt: &Stmt) -> Result<(), LoxResult> {
        stmt.accept(self)
    }
    fn is_truthy(&self, literal: &Literal) -> bool {
        !matches!(literal, Literal::Nil | Literal::Boolean(false))
    }

    pub fn interprete(&self, statements: &[Stmt]) -> bool {
        let mut success = true;
        for statement in statements {
            if self.execute(statement).is_err() {
                success = false;
                break;
            }
        }
        success
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::token::Token;
    use rstest::*;

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

    fn make_var_identifier(lexeme: &str) -> Token {
        make_token_operator(TokenType::Identifier, lexeme)
    }

    #[test]
    fn test_variable_expression_defined() {
        let interp = Interpreter::new();

        let var_name = make_var_identifier("my_var");
        let var_statement = Stmt::Var(StmtVar {
            name: var_name.clone(),
            initializer: Some(*make_literal(Literal::Number(42.0))),
        });
        assert!(interp.execute(&var_statement).is_ok());

        let var_expression = ExprVariable {
            name: make_var_identifier("my_var"),
        };
        assert_eq!(
            interp.visit_variable_expr(&var_expression).unwrap(),
            Literal::Number(42.0)
        );
    }

    #[test]
    fn test_variable_expression_undefined() {
        let interp = Interpreter::new();

        let var_name = make_var_identifier("my_var");
        let var_expression = ExprVariable { name: var_name };
        assert!(interp.visit_variable_expr(&var_expression).is_err());
    }

    #[test]
    fn test_variable_declaration_statement_nil() {
        let interp = Interpreter::new();
        let var = make_var_identifier("my_var");

        let var_statement = Stmt::Var(StmtVar {
            name: var.clone(),
            initializer: None,
        });

        let result = interp.execute(&var_statement);
        assert!(result.is_ok());
        assert_eq!(
            interp.environment.borrow().borrow().get(&var).unwrap(),
            Literal::Nil
        );
    }

    #[test]
    fn test_variable_declaration_statement() {
        let interp = Interpreter::new();

        let var = make_var_identifier("my_var");
        let expr = make_literal(Literal::Number(42.0));

        let var_statement = Stmt::Var(StmtVar {
            name: var.clone(),
            initializer: Some(*expr),
        });

        let result = interp.execute(&var_statement);

        assert!(result.is_ok());
        assert_eq!(
            interp.environment.borrow().borrow().get(&var).unwrap(),
            Literal::Number(42.0)
        );
    }
    #[rstest]
    #[case::minus_for_number (make_literal(Literal::Number(42.0)), Some(Literal::Number(-42.0)))]
    #[case::minus_for_string (make_literal(Literal::String("some".to_string())), Some(Literal::Nil))]
    fn test_unary_minus_for_number(#[case] right: Box<Expr>, #[case] expected: Option<Literal>) {
        let interp = Interpreter::new();
        let unary_minus = Expr::Unary(ExprUnary {
            operator: make_token_operator(TokenType::Minus, "-"),
            right,
        });
        let result = interp.evaluate(&unary_minus);
        assert!(result.is_ok());
        assert_eq!(result.ok(), expected)
    }

    #[rstest]
    #[case::bang_true(make_literal(Literal::Boolean(true)), false)]
    #[case::bang_false(make_literal(Literal::Boolean(false)), true)]
    #[case::bang_nil(make_literal(Literal::Nil), true)]
    fn test_unary_bang(#[case] input: Box<Expr>, #[case] expected: bool) {
        let interp = Interpreter::new();
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
        let interp = Interpreter::new();
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
        let interp = Interpreter::new();
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
        let interp = Interpreter::new();
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
        let interp = Interpreter::new();
        let binary_star = make_binary_expression(
            make_literal(Literal::Number(2.0)),
            make_literal(Literal::Number(2.0)),
            make_token_operator(TokenType::Star, "*"),
        );
        let result = interp.evaluate(&binary_star);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Number(4.0)));
    }

    #[rstest]
    #[case::add_numbers(
        make_literal(Literal::Number(2.0)),
        make_literal(Literal::Number(2.0)),
        Some(Literal::Number(4.0))
    )]
    #[case::concatenates_strings (make_literal(Literal::String("Hello".to_string())), make_literal(Literal::String(" world!".to_string())),Some(Literal::String("Hello world!".to_string())))]
    #[case::number_and_string (make_literal(Literal::Number(3.0)), make_literal(Literal::String("123".to_string())),Some(Literal::String("3123".to_string())))]
    #[case::string_and_number (make_literal(Literal::String("123".to_string())), make_literal(Literal::Number(4.0)),Some(Literal::String("1234".to_string())))]
    fn test_binary_addition(
        #[case] left: Box<Expr>,
        #[case] right: Box<Expr>,
        #[case] expected: Option<Literal>,
    ) {
        let interp = Interpreter::new();
        let binary_plus =
            make_binary_expression(left, right, make_token_operator(TokenType::Plus, "+"));
        let result = interp.evaluate(&binary_plus);

        assert!(result.is_ok());
        assert_eq!(result.ok(), expected);
    }

    #[test]
    fn test_binary_greater_than() {
        let interp = Interpreter::new();
        let binary_concat = make_binary_expression(
            make_literal(Literal::Number(6.0)),
            make_literal(Literal::Number(5.0)),
            make_token_operator(TokenType::Greater, ">"),
        );

        let result = interp.evaluate(&binary_concat);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
    }
    #[rstest]
    #[case::greater_than(make_literal(Literal::Number(6.0)), make_literal(Literal::Number(5.0)))]
    #[case::greater_equal(make_literal(Literal::Number(6.0)), make_literal(Literal::Number(6.0)))]
    fn test_binary_greater_equal(#[case] left: Box<Expr>, #[case] right: Box<Expr>) {
        let interp = Interpreter::new();
        let binary_compare = make_binary_expression(
            left,
            right,
            make_token_operator(TokenType::GreaterEqual, ">="),
        );

        let result = interp.evaluate(&binary_compare);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
    }
    #[rstest]
    #[case::less_than(make_literal(Literal::Number(5.0)), make_literal(Literal::Number(6.0)))]
    #[case::less_equal(make_literal(Literal::Number(5.0)), make_literal(Literal::Number(5.0)))]
    fn test_binary_less_equal(#[case] left: Box<Expr>, #[case] right: Box<Expr>) {
        let interp = Interpreter::new();
        let binary_less =
            make_binary_expression(left, right, make_token_operator(TokenType::LessEqual, "<="));

        let result = interp.evaluate(&binary_less);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
    }
    #[test]
    fn test_binary_less() {
        let interp = Interpreter::new();
        let binary_less = make_binary_expression(
            make_literal(Literal::Number(5.0)),
            make_literal(Literal::Number(6.0)),
            make_token_operator(TokenType::Less, "<"),
        );

        let result = interp.evaluate(&binary_less);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
    }
    #[rstest]
    #[case::two_numbers(make_literal(Literal::Number(6.0)), make_literal(Literal::Number(4.0)))]
    #[case::two_strings (make_literal(Literal::String("abc".to_string())), make_literal(Literal::String("bta".to_string())))]
    #[case::two_booleans(
        make_literal(Literal::Boolean(true)),
        make_literal(Literal::Boolean(false))
    )]
    #[case::mixed_types(
        make_literal(Literal::Number(3.0)),
        make_literal(Literal::Boolean(false))
    )]
    #[case::mixed_types(make_literal(Literal::Nil), make_literal(Literal::Boolean(false)))]
    #[case::mixed_types(make_literal(Literal::Nil), make_literal(Literal::Number(2.3)))]
    #[case::mixed_types (make_literal(Literal::String("false".to_string())), make_literal(Literal::Boolean(false)))]
    fn test_binary_bang_equal_inputs(#[case] left: Box<Expr>, #[case] right: Box<Expr>) {
        let interp = Interpreter::new();
        let binary_bang_equal =
            make_binary_expression(left, right, make_token_operator(TokenType::BangEqual, "!="));

        let result = interp.evaluate(&binary_bang_equal);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
    }
    #[rstest]
    #[case::two_numbers(make_literal(Literal::Number(6.0)), make_literal(Literal::Number(6.0)))]
    #[case::two_strings (make_literal(Literal::String("abc".to_string())), make_literal(Literal::String("abc".to_string())))]
    #[case::two_booleans(
        make_literal(Literal::Boolean(true)),
        make_literal(Literal::Boolean(true))
    )]
    #[case::two_nulls(make_literal(Literal::Nil), make_literal(Literal::Nil))]
    fn test_binary_equal_equal(#[case] left: Box<Expr>, #[case] right: Box<Expr>) {
        let interp = Interpreter::new();
        let binary_equal =
            make_binary_expression(left, right, make_token_operator(TokenType::Equals, "=="));

        let result = interp.evaluate(&binary_equal);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Boolean(true)));
    }
}
