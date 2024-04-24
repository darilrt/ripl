use std::cell::RefCell;

use crate::{
    ast::{Ast, BinaryOp, Expr, Stmt},
    token::{Token, TokenKind},
};

#[derive(Clone)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: RefCell<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: RefCell::new(0),
        }
    }

    pub fn parse(&self) -> Ast {
        Ast::Stmt(self.parse_statement())
    }

    fn parse_statement(&self) -> Stmt {
        Stmt::Expr(self.parse_expr())
    }

    // expr = additive
    fn parse_expr(&self) -> Expr {
        self.parse_additive()
    }

    // additive = multiplicative ( ( '+' | '-' ) additive )
    fn parse_additive(&self) -> Expr {
        let mut lhs = self.parse_multiplicative();

        if self.is_peek(TokenKind::Operator, "+") || self.is_peek(TokenKind::Operator, "-") {
            let op = self.next().unwrap();
            let rhs = Box::new(self.parse_additive());

            let op = match op.lit.as_str() {
                "+" => BinaryOp::Add,
                "-" => BinaryOp::Sub,
                _ => panic!("Unknown operator: {}", op.lit),
            };

            lhs = Expr::BinaryOp(Box::new(lhs), op, rhs);
        }

        lhs
    }

    // multiplicative = primary ( ( '*' | '/' ) multiplicative )
    fn parse_multiplicative(&self) -> Expr {
        let mut lhs = self.parse_primary();

        if self.is_peek(TokenKind::Operator, "*") || self.is_peek(TokenKind::Operator, "/") {
            let op = self.next().unwrap();
            let rhs = Box::new(self.parse_multiplicative());

            let op = match op.lit.as_str() {
                "*" => BinaryOp::Mul,
                "/" => BinaryOp::Div,
                _ => panic!("Unknown operator: {}", op.lit),
            };

            lhs = Expr::BinaryOp(Box::new(lhs), op, rhs);
        }

        lhs
    }

    // primary = integer
    fn parse_primary(&self) -> Expr {
        if self.is_peek(TokenKind::Integer, "") {
            Expr::Integer(self.next().unwrap().lit.parse().unwrap())
        } else if self.is_peek(TokenKind::Float, "") {
            Expr::Float(self.next().unwrap().lit.parse().unwrap())
        } else if self.is_peek(TokenKind::String, "") {
            Expr::String(self.next().unwrap().lit.clone())
        } else if self.is_peek(TokenKind::Symbol, "(") {
            self.next();
            let expr = self.parse_expr();
            if self.is_peek(TokenKind::Symbol, ")") {
                self.next();
                expr
            } else {
                panic!("Expected ')', but got {:?}", self.peek());
            }
        } else {
            panic!("Unexpected token: {:?}", self.peek());
        }
    }

    fn is_peek(&self, kind: TokenKind, lit: &str) -> bool {
        self.peek().map_or(false, |t| {
            t.kind == kind && (lit.is_empty() || t.lit == lit)
        })
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(*self.pos.borrow())
    }

    fn next(&self) -> Option<&Token> {
        *self.pos.borrow_mut() += 1;
        self.tokens.get(*self.pos.borrow_mut() - 1)
    }
}
