use crate::error::*;
use crate::expr::*;
use crate::literal::*;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> Result<String, LoxError> {
        expr.accept(self)
    }
    fn parentesize(&self, name: &str, exprs: &[&Expr]) -> Result<String, LoxError> {
        let mut result = String::new();

        result.push('(');
        result.push_str(name);
        for expr in exprs {
            result.push(' ');
            result.push_str(&expr.accept(self)?);
        }
        result.push(')');

        Ok(result)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_variable_expr(&self, expr: &ExprVariable) -> Result<String, LoxError> {
        todo!("Needs to be implemented!")
    }    


    fn visit_binary_expr(&self, expr: &ExprBinary) -> Result<String, LoxError> {
        self.parentesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&self, expr: &ExprGrouping) -> Result<String, LoxError> {
        self.parentesize("group", &[&expr.expression])
    }

    fn visit_literal_expr(&self, expr: &ExprLiteral) -> Result<String, LoxError> {
        if let Some(value) = &expr.value {
            match value {
                Literal::Boolean(a) => {
                    if *a {
                        Ok("True".to_string())
                    } else {
                        Ok("False".to_string())
                    }
                }
                Literal::String(a) => Ok(a.to_string()),
                Literal::Nil => Ok("nil".to_string()),
                Literal::Number(a) => Ok(a.to_string()),
                Literal::Identifier(a) => Ok(a.to_string()),
            }
        } else {
            Ok("nil".to_string())
        }
    }

    fn visit_unary_expr(&self, expr: &ExprUnary) -> Result<String, LoxError> {
        self.parentesize("unary", &[&expr.right])
    }
}
