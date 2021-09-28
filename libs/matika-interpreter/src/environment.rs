use std::{collections::HashMap, rc::Rc};

use crate::functions::{Factors, Pi, Plot, Sin};
use crate::value::Value;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn global() -> Self {
        let mut values = HashMap::new();

        let pi = Rc::new(Pi);
        let sin = Rc::new(Sin);
        let factors = Rc::new(Factors);
        let plot = Rc::new(Plot);

        values.insert(String::from("pi"), Value::Function(pi));
        values.insert(String::from("sin"), Value::Function(sin));
        values.insert(String::from("factors"), Value::Function(factors));
        values.insert(String::from("plot"), Value::Function(plot));

        Self {
            values,
            enclosing: None,
        }
    }

    pub fn from_enclosing(enclosing: Environment) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        match self.values.get(name) {
            value @ Some(_) => value.cloned(),
            _ => self.enclosing.as_ref().and_then(|env| env.get(name)),
        }
    }
}
