use crate::callable::*;
use crate::environment::*;
use crate::error::*;
use crate::interpreter::*;
use crate::literal::*;
use crate::stmt::*;
use crate::token::*;
use std::rc::Rc;

pub struct LoxFunction {
    name: Token,
    body: Rc<Vec<Stmt>>,
    params: Rc<Vec<Token>>,
}

impl LoxFunction {
    pub fn new(declaration: &StmtFunction) -> Self {
        Self {
            name: declaration.name.clone(),
            body: Rc::clone(&declaration.body),
            params: Rc::clone(&declaration.params),
        }
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn to_string(&self) -> String {
        self.name.as_string().into()
    }

    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: Vec<Literal>,
    ) -> Result<Literal, LoxResult> {
        let mut environment = Environment::nested_in(Rc::clone(&interpreter.globals));

        for (parameter, argument) in self.params.iter().zip(arguments.iter()) {
            environment.define(parameter.as_string(), argument.clone());
        }
        interpreter.execute_block(&self.body, environment)?;

        Ok(Literal::Nil)
    }
}
