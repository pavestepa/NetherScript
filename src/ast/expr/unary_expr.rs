use crate::{ast::Expr, lexer::Token};

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op: UnaryOperator,
    pub expr: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(op: UnaryOperator, expr: Box<Expr>) -> Self {
        Self { op, expr }
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Minus,
    Plus,
    Not,
    BitNot,
}

impl UnaryOperator {
    pub fn from_token(token: Token) -> Option<Self> {
        match token {
            Token::Minus => Some(UnaryOperator::Minus),
            Token::Plus => Some(UnaryOperator::Plus),
            Token::Not => Some(UnaryOperator::Not),
            Token::BitNot => Some(UnaryOperator::BitNot),
            _ => None,
        }
    }
}
