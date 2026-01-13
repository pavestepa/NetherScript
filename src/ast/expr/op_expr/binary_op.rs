use crate::{ast::Expr, lexer::Token};

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Plus(Box<Expr>),
    Minus(Box<Expr>),
    Star(Box<Expr>),
    Slash(Box<Expr>),
    Percent(Box<Expr>),
}

impl BinaryOp {
    pub fn from(token: Token, expr: Expr) -> Self {
        match token {
            Token::Plus => BinaryOp::Plus(Box::new(expr)),
            Token::Minus => BinaryOp::Minus(Box::new(expr)),
            Token::Star => BinaryOp::Star(Box::new(expr)),
            Token::Slash => BinaryOp::Slash(Box::new(expr)),
            e => panic!("Token {:?} is not suitable for binary expression", e),
        }
    }
}
