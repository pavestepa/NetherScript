use crate::syntax::{
    ast::{ast::Ast, Expr},
    lexer::TokenKind,
};

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Minus(Box<Ast<Expr>>),
    Plus(Box<Ast<Expr>>),
    Not(Box<Ast<Expr>>),
    BitNot(Box<Ast<Expr>>),
}

impl UnaryOp {
    pub fn from(token: TokenKind, expr: Ast<Expr>) -> Self {
        match token {
            TokenKind::Minus => UnaryOp::Minus(Box::new(expr)),
            TokenKind::Plus => UnaryOp::Plus(Box::new(expr)),
            TokenKind::Not => UnaryOp::Not(Box::new(expr)),
            TokenKind::BitNot => UnaryOp::BitNot(Box::new(expr)),
            e => panic!("Token {:?} is not suitable for unary expression", e),
        }
    }
}
