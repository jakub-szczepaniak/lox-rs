use crate::error::LoxResult;
use crate::interpreter::*;
use crate::literal::*;
use core::fmt::Debug;
use std::rc::Rc;
#[derive(Clone)]
pub struct Callable {
    func: Rc<dyn LoxCallable>,
    arity: usize,
}

impl Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<callable>")
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
}

impl LoxCallable for Callable {
    fn call(
        &self,
        interpreter: &Interpreter,
        argument: Vec<Literal>,
    ) -> Result<Literal, LoxResult> {
        Ok(Literal::Nil)
    }

    fn arity(&self) -> usize {
        self.arity
    }
}
