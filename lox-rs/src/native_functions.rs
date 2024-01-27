use crate::callable::*;
use crate::error::LoxResult;
use crate::interpreter::*;
use crate::literal::Literal;
use std::time::SystemTime;
pub struct LoxClock {}

impl LoxCallable for LoxClock {
    fn call(
        &self,
        _interpreter: &Interpreter,
        _arguments: Vec<Literal>,
    ) -> Result<Literal, LoxResult> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => Ok(Literal::Number(n.as_millis() as f64)),
            Err(e) => Err(LoxResult::system_error(&format!(
                "System clock returned invalid value: {:?} ",
                e
            ))),
        }
    }

    fn arity(&self) -> usize {
        0
    }
}
