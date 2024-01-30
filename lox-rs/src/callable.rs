use crate::error::LoxResult;
use crate::interpreter::*;
use crate::literal::*;
use core::fmt::{Debug, Display};
use std::rc::Rc;
#[derive(Clone)]
pub struct Callable {
    pub func: Rc<dyn LoxCallable>,
}

impl Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", LoxCallable::to_string(self))
    }
}

impl Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", LoxCallable::to_string(self))
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func)
    }
}

pub trait LoxCallable {
    fn call(&self, interpreter: &Interpreter, argument: Vec<Literal>)
        -> Result<Literal, LoxResult>;
    fn arity(&self) -> usize;

    fn to_string(&self) -> String;
}

impl LoxCallable for Callable {
    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: Vec<Literal>,
    ) -> Result<Literal, LoxResult> {
        self.func.call(interpreter, arguments)
    }

    fn arity(&self) -> usize {
        self.func.arity()
    }

    fn to_string(&self) -> String {
        self.func.to_string()
    }
}
