use std::rc::Rc;
use std::{fmt, ops};

use crate::callable::Callable;

#[derive(Debug, Clone)]
pub enum Value {
    Unbound(String),
    Number(f64),
    List(Vec<f64>),
    Function(Rc<dyn Callable>),
}

impl Value {
    pub fn powf(&self, other: Self) -> Self {
        match self {
            Value::Number(number) => match other {
                Value::Number(other) => Value::Number(number.powf(other)),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

impl From<Value> for f64 {
    fn from(value: Value) -> Self {
        match value {
            Value::Number(num) => num,
            _ => 0.0,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Unbound(ident) => write!(f, "{}", ident),
            Value::Number(number) => write!(f, "{}", number),
            Value::Function(fnc) => write!(f, "{}", fnc),
            Value::List(numbers) => {
                write!(f, "{:?}", numbers)
            }
        }
    }
}

impl ops::Neg for Value {
    type Output = Self;

    fn neg(self) -> Self {
        if let Value::Number(number) = self {
            return Value::Number(-number);
        }

        panic!()
    }
}

impl ops::Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            Value::Number(number) => match other {
                Value::Number(other) => Value::Number(number + other),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

impl ops::Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self {
            Value::Number(number) => match other {
                Value::Number(other) => Value::Number(number - other),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

impl ops::Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match self {
            Value::Number(number) => match other {
                Value::Number(other) => Value::Number(number * other),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

impl ops::Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match self {
            Value::Number(number) => match other {
                Value::Number(other) => Value::Number(number / other),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}
