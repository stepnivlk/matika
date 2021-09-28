use std::rc::Rc;

use crate::functions::Function;
use crate::value::Value;
use crate::{
    environment::Environment,
    expr::Expr,
    plotter::Plotter,
    stmt::Stmt,
    token::{LiteralKind, Token, TokenKind},
    visitor::{Acceptor, Visitor},
};

pub struct Interpreter {
    pub environment: Environment,
    pub plotter: Option<Box<dyn Plotter>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::global(),
            plotter: None,
        }
    }

    pub fn with_plotter(mut self, plotter: Box<dyn Plotter>) -> Self {
        self.plotter = Some(plotter);

        self
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Value {
        let mut val = Value::Number(0.0);

        for stmt in statements {
            val = self.execute(&stmt);
        }

        val
    }

    fn execute(&mut self, stmt: &Stmt) -> Value {
        stmt.accept(self)
    }

    pub fn evaluate_inner(&mut self, expr: &Expr, environment: Environment) -> Value {
        let previous = self.environment.clone();

        self.environment = environment;

        let result = self.evaluate(expr);

        self.environment = previous;

        result
    }

    fn evaluate(&mut self, expr: &Expr) -> Value {
        expr.accept(self)
    }

    fn visit_unary_expr(&mut self, op: &Token, right: &Expr) -> Value {
        let right = self.evaluate(&right);

        match op.kind {
            TokenKind::Minus => -right,
            _ => panic!(),
        }
    }

    fn visit_binary_expr(&mut self, left: &Expr, op: &Token, right: &Expr) -> Value {
        let left = self.evaluate(&left);
        let right = self.evaluate(&right);

        let ret = match op.kind {
            TokenKind::Plus => left + right,
            TokenKind::Minus => left - right,
            TokenKind::Star => left * right,
            TokenKind::Slash => left / right,
            TokenKind::Caret => left.powf(right),
            _ => panic!(),
        };

        ret
    }
}

impl Visitor<Stmt> for &mut Interpreter {
    type Result = Value;

    fn visit(&mut self, stmt: &Stmt) -> Self::Result {
        match stmt {
            Stmt::Expression(expr) => self.evaluate(expr),
            Stmt::Print(expr) => {
                let value = self.evaluate(expr);

                println!("{}", value);

                Value::Number(0.0)
            }
            Stmt::Variable { name, initializer } => {
                let value = self.evaluate(&initializer);

                self.environment.define(name.lexeme.clone(), value);

                Value::Number(0.0)
            }
            Stmt::Function(fnc) => {
                let function = Rc::new(Function {
                    declaration: fnc.clone(),
                });

                self.environment
                    .define(fnc.name.lexeme.clone(), Value::Function(function));

                Value::Number(0.0)
            }
        }
    }
}

impl Visitor<Expr> for &mut Interpreter {
    type Result = Value;

    fn visit(&mut self, expr: &Expr) -> Self::Result {
        match expr {
            Expr::Literal(LiteralKind::Number(value)) => Value::Number(*value),
            Expr::Grouping(expr) => self.evaluate(&expr),
            Expr::Unary { op, right } => self.visit_unary_expr(op, right),
            Expr::Binary { left, op, right } => self.visit_binary_expr(left, op, right),
            Expr::Call { callee, args } => {
                let callee = self.evaluate(callee);

                let args: Vec<Value> = args.iter().map(|arg| self.evaluate(arg)).collect();

                match callee {
                    Value::Function(fnc) => {
                        if args.len() != fnc.arity() as usize {
                            panic!("Arity mismatch")
                        }
                        fnc.call(self, args)
                    }
                    _ => panic!("Not a function"),
                }
            }
            Expr::Variable(variable) => {
                if let Some(value) = self.environment.get(&variable.lexeme) {
                    value
                } else {
                    let value = Value::Unbound(variable.lexeme.clone());

                    self.environment
                        .define(variable.lexeme.clone(), value.clone());

                    value
                }
            }
        }
    }
}
