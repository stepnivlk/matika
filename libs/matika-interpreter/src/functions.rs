use std::fmt;

use crate::callable::Callable;
use crate::environment::Environment;
use crate::stmt::Fnc;
use crate::value::Value;
use crate::{interpreter, Interpreter};

#[derive(Debug)]
pub struct Function {
    pub declaration: Fnc,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fnc:{}>", self.declaration.name.lexeme)
    }
}

impl Callable for Function {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Value {
        let mut env = Environment::from_enclosing(interpreter.environment.clone());

        for (idx, param) in self.declaration.params.iter().enumerate() {
            let arg = args[idx].clone();

            env.define(param.lexeme.clone(), arg);
        }

        interpreter.evaluate_inner(&self.declaration.body, env)
    }

    fn arity(&self) -> u64 {
        self.declaration.params.len() as u64
    }
}

#[derive(Debug)]
pub struct Pi;

impl Callable for Pi {
    fn call(&self, _: &mut Interpreter, _: Vec<Value>) -> Value {
        Value::Number(std::f64::consts::PI)
    }

    fn arity(&self) -> u64 {
        0
    }
}

impl fmt::Display for Pi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fnc:pi>")
    }
}

#[derive(Debug)]
pub struct Sin;

impl fmt::Display for Sin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fnc:sin>")
    }
}

impl Callable for Sin {
    fn call(&self, _: &mut Interpreter, args: Vec<Value>) -> Value {
        let arg = &args[0];

        if let Value::Number(num) = arg {
            Value::Number(num.sin())
        } else {
            panic!()
        }
    }

    fn arity(&self) -> u64 {
        1
    }
}

#[derive(Debug)]
pub struct Factors;

impl fmt::Display for Factors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fnc:factors>")
    }
}

impl Callable for Factors {
    fn call(&self, _: &mut Interpreter, args: Vec<Value>) -> Value {
        let arg = &args[0];

        if let Value::Number(num) = arg {
            let num = *num as u64;
            let factors = (1..num + 1)
                .into_iter()
                .filter(|&x| num % x == 0)
                .map(|x| x as f64)
                .collect::<Vec<f64>>();

            Value::List(factors)
        } else {
            panic!()
        }
    }

    fn arity(&self) -> u64 {
        1
    }
}

#[derive(Debug)]
pub struct Plot;

impl fmt::Display for Plot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fnc:plot>")
    }
}

impl Callable for Plot {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Value {
        let arg = &args[0];

        if let Value::Function(fnc) = arg {
            let mut points: Vec<(f32, f32)> = vec![];

            for x in -10..10 {
                let y: f64 = fnc.call(interpreter, vec![Value::Number(x as f64)]).into();

                points.push((x as f32, y as f32))
            }

            interpreter
                .plotter
                .as_ref()
                .map(|plotter| plotter.plot(points));
        } else {
            panic!("Not a function");
        }

        Value::Number(0.0)
    }

    fn arity(&self) -> u64 {
        1
    }
}
