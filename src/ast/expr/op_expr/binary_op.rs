use crate::{ast::Expr, lexer::TokenKind};

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Plus(Box<Expr>),
    Minus(Box<Expr>),
    Star(Box<Expr>),
    Slash(Box<Expr>),
    Percent(Box<Expr>),
}

impl BinaryOp {
    pub fn from(token: TokenKind, expr: Expr) -> Self {
        match token {
            TokenKind::Plus => BinaryOp::Plus(Box::new(expr)),
            TokenKind::Minus => BinaryOp::Minus(Box::new(expr)),
            TokenKind::Star => BinaryOp::Star(Box::new(expr)),
            TokenKind::Slash => BinaryOp::Slash(Box::new(expr)),
            e => panic!("Token {:?} is not suitable for binary expression", e),
        }
    }
}
