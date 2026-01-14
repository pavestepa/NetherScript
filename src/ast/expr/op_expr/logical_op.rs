use crate::{ast::Expr, lexer::TokenKind};

#[derive(Debug, Clone)]
pub enum LogicalOp {
    Equals(Box<Expr>),
    NotEquals(Box<Expr>),
    Less(Box<Expr>),
    Greater(Box<Expr>),
    LessEqual(Box<Expr>),
    GreaterEqual(Box<Expr>),
}

impl LogicalOp {
    pub fn from(token: TokenKind, expr: Expr) -> Self {
        match token {
            TokenKind::Equals => LogicalOp::Equals(Box::new(expr)),
            TokenKind::NotEquals => LogicalOp::NotEquals(Box::new(expr)),
            TokenKind::Less => LogicalOp::Less(Box::new(expr)),
            TokenKind::Greater => LogicalOp::Greater(Box::new(expr)),
            TokenKind::LessEqual => LogicalOp::LessEqual(Box::new(expr)),
            TokenKind::GreaterEqual => LogicalOp::GreaterEqual(Box::new(expr)),
            e => panic!("Token {:?} is not suitable for logical expression", e),
        }
    }
}
