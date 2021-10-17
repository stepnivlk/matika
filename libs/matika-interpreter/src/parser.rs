use crate::{expr::Expr, stmt::{Fnc, Stmt}, token::{LiteralKind, Token, TokenKind}};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = vec![];

        while !self.is_end() {
            statements.push(self.declaration());
        }

        statements
    }

    fn declaration(&mut self) -> Stmt {
        let stmt = self.statement();

        if let Some(_) = self.matches(vec![TokenKind::Equal]) {
            let initializer = self.expression();

            if let Stmt::Expression(Expr::Variable(var)) = stmt {
                return Stmt::Variable {
                    name: var,
                    initializer,
                };
            } else if let Stmt::Expression(Expr::Call { callee, args }) = stmt.clone() {
                let mut params: Vec<Token> = vec![];

                for var in args {
                    if let Expr::Variable(var) = var {
                        params.push(var);
                    }
                }

                if let Expr::Variable(var) = *callee {
                    return Stmt::Function(Fnc {
                        name: var,
                        params,
                        body: initializer,
                    });
                }
            } else {
                panic!("Declaration not supported")
            }
        }

        stmt
    }

    fn statement(&mut self) -> Stmt {
        if let Some(_) = self.matches(vec![TokenKind::Print]) {
            return self.print_statement();
        }

        Stmt::Expression(self.expression())
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression();

        Stmt::Print(value)
    }

    fn expression(&mut self) -> Expr {
        self.term()
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while let Some(op) = self.matches(vec![TokenKind::Minus, TokenKind::Plus]) {
            let right = self.factor();

            expr = Expr::Binary {
                left: Box::new(expr),
                right: Box::new(right),
                op,
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.power();

        while let Some(op) = self.matches(vec![TokenKind::Star, TokenKind::Slash]) {
            let right = self.power();

            expr = Expr::Binary {
                left: Box::new(expr),
                right: Box::new(right),
                op,
            };
        }

        expr
    }

    fn power(&mut self) -> Expr {
        let mut expr = self.unary();

        while let Some(op) = self.matches(vec![TokenKind::Caret]) {
            let right = self.primary();

            expr = Expr::Binary {
                left: Box::new(expr),
                right: Box::new(right),
                op,
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if let Some(op) = self.matches(vec![TokenKind::Minus]) {
            let right = self.unary();

            return Expr::Unary {
                op,
                right: Box::new(right),
            };
        }

        self.call()
    }

    fn call(&mut self) -> Expr {
        let mut expr = self.primary();

        while let Some(_) = self.matches(vec![TokenKind::LeftParen]) {
            let mut args: Vec<Expr> = vec![];

            if !self.check(TokenKind::RightParen) {
                loop {
                    args.push(self.expression());

                    if let None = self.matches(vec![TokenKind::Comma]) {
                        break;
                    }
                }
            }

            self.consume(TokenKind::RightParen);

            match expr {
                Expr::Literal(LiteralKind::Number(_)) => {
                    if args.len() == 1 {
                        expr = Expr::Binary {
                            left: Box::new(expr),
                            right: Box::new(args[0].clone()),
                            op: Token::star(),
                        };
                    } else {
                        panic!();
                    }
                },
                _ => {
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                    };
                }
            }

        }

        expr
    }

    fn primary(&mut self) -> Expr {
        if let Some(num) = self.matches(vec![TokenKind::Number]) {
            return Expr::Literal(num.literal.unwrap());
        }

        if let Some(var) = self.matches(vec![TokenKind::Identifier]) {
            return Expr::Variable(var);
        }

        if let Some(_) = self.matches(vec![TokenKind::LeftParen]) {
            let expr = self.expression();

            self.consume(TokenKind::RightParen);

            return Expr::Grouping(Box::new(expr));
        }

        panic!()
    }

    fn consume(&mut self, kind: TokenKind) -> &Token {
        if self.check(kind) {
            return self.advance();
        }

        panic!();
    }

    fn matches(&mut self, forms: Vec<TokenKind>) -> Option<Token> {
        for kind in forms {
            if self.check(kind) {
                return Some(self.advance().clone());
            }
        }

        None
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_end() {
            return false;
        }

        self.peek().map(|token| token.kind == kind).unwrap_or(false)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_end() {
            self.current = self.current + 1;
        }

        self.previous()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn is_end(&self) -> bool {
        match self.peek() {
            Some(token) => token.kind == TokenKind::Eof,
            None => true,
        }
    }
}
