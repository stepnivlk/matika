use crate::token::Token;
use crate::{
    visitor::{Acceptor, Visitor},
    Expr,
};

#[derive(Debug)]
pub struct Variable {
    pub name: Token,
    pub initializer: Expr,
}

#[derive(Debug, Clone)]
pub struct Fnc {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
    Function(Fnc),
    Variable { name: Token, initializer: Expr },
}

impl Acceptor<Stmt> for Stmt {
    fn accept<V: Visitor<Stmt>>(&self, mut visitor: V) -> V::Result {
        visitor.visit(self)
    }
}
