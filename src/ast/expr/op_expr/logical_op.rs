use crate::{ast::Expr, lexer::Token};

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
    pub fn from(token: Token, expr: Expr) -> Self {
        match token {
            Token::Equals => LogicalOp::Equals(Box::new(expr)),
            Token::NotEquals => LogicalOp::NotEquals(Box::new(expr)),
            Token::Less => LogicalOp::Less(Box::new(expr)),
            Token::Greater => LogicalOp::Greater(Box::new(expr)),
            Token::LessEqual => LogicalOp::LessEqual(Box::new(expr)),
            Token::GreaterEqual => LogicalOp::GreaterEqual(Box::new(expr)),
            e => panic!("Token {:?} is not suitable for logical expression", e),
        }
    }
}
