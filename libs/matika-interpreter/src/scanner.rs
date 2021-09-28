use std::collections::HashMap;

use crate::token::{LiteralKind, Token, TokenKind};

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenKind>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();

        keywords.insert("print".into(), TokenKind::Print);

        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
            keywords,
        }
    }

    pub fn scan(&mut self) -> &Vec<Token> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::eof());

        &self.tokens
    }

    fn scan_token(&mut self) {
        let ch = self.advance();

        match ch {
            Some('(') => self.add_token(TokenKind::LeftParen, None),
            Some(')') => self.add_token(TokenKind::RightParen, None),
            Some('.') => self.add_token(TokenKind::Dot, None),
            Some('-') => self.add_token(TokenKind::Minus, None),
            Some('+') => self.add_token(TokenKind::Plus, None),
            Some('*') => self.add_token(TokenKind::Star, None),
            Some('^') => self.add_token(TokenKind::Caret, None),
            Some('!') => self.add_token(TokenKind::Bang, None),
            Some('=') => self.add_token(TokenKind::Equal, None),
            Some(',') => self.add_token(TokenKind::Comma, None),
            Some('<') => {
                let token = if self.matches('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };

                self.add_token(token, None);
            }
            Some('>') => {
                let token = if self.matches('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };

                self.add_token(token, None);
            }
            Some('/') => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenKind::Slash, None)
                }
            }

            Some(' ') => (),
            Some('\r') => (),
            Some('\t') => (),

            Some('\n') => self.line += 1,

            Some(c) if self.is_digit(c) => self.number(),

            Some(c) if self.is_alpha(c) => self.identifier(),

            Some(_) => panic!("Unexpected character"),
            None => (),
        }
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_digit(c) || self.is_alpha(c)
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        let kind = self
            .keywords
            .get(text)
            .unwrap_or(&TokenKind::Identifier)
            .clone();

        self.add_token(kind, None);
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = &self.source[self.start..self.current];
        let value = value.parse::<f64>().unwrap();

        self.add_token(TokenKind::Number, Some(LiteralKind::Number(value)));

        if self.is_alpha(self.peek()) {
            let token = Token {
                kind: TokenKind::Star,
                lexeme: "*".to_string(),
                literal: None,
            };

            self.tokens.push(token);
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        match self.source.chars().nth(self.current + 1) {
            Some(ch) => ch,
            None => '\n',
        }
    }

    fn peek(&self) -> char {
        if self.is_end() {
            return '\0';
        }

        match self.source.chars().nth(self.current) {
            Some(ch) => ch,
            None => '\0',
        }
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }

        match self.source.chars().nth(self.current) {
            Some(ch) => {
                if ch == expected {
                    self.current += 1;
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn add_token(&mut self, kind: TokenKind, literal: Option<LiteralKind>) {
        let text = &self.source[self.start..self.current];

        let token = Token {
            kind,
            lexeme: text.to_string(),
            literal,
        };

        self.tokens.push(token);
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
