use crate::{ast::Expr, lexer::Token};

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Minus(Box<Expr>),
    Plus(Box<Expr>),
    Not(Box<Expr>),
    BitNot(Box<Expr>),
}

impl UnaryOp {
    pub fn from(token: Token, expr: Expr) -> Self {
        match token {
            Token::Minus => UnaryOp::Minus(Box::new(expr)),
            Token::Plus => UnaryOp::Plus(Box::new(expr)),
            Token::Not => UnaryOp::Not(Box::new(expr)),
            Token::BitNot => UnaryOp::BitNot(Box::new(expr)),
            e => panic!("Token {:?} is not suitable for unary expression", e),
        }
    }
}
