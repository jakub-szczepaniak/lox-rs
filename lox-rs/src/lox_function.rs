use crate::callable::*;
use crate::error::*;
use crate::interpreter::*;
use crate::literal::*;

struct LoxFunction {}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        "ast".to_string()
    }

    fn call(
        &self,
        interpreter: &Interpreter,
        argument: Vec<Literal>,
    ) -> Result<Literal, LoxResult> {
        Ok(Literal::Nil)
    }
}
