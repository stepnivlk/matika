use std::fmt;

use crate::value::Value;
use crate::Interpreter;

pub trait Callable: fmt::Debug + fmt::Display {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Value;

    fn arity(&self) -> u64;
}
