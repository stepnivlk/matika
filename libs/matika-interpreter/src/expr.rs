use crate::{
    token::{LiteralKind, Token},
    visitor::{Acceptor, Visitor},
};

#[derive(Clone, Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(LiteralKind),
    Variable(Token),
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
}

impl Acceptor<Expr> for Expr {
    fn accept<V: Visitor<Expr>>(&self, mut visitor: V) -> V::Result {
        visitor.visit(self)
    }
}
