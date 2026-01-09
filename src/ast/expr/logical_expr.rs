use crate::lexer::Token;

use super::Expr;

#[derive(Debug, Clone)]
pub struct LogicalExpr {
    left: Box<Expr>,
    op: LogicalOperator,
    right: Box<Expr>,
}
impl LogicalExpr {
    pub fn new(left: Expr, op: LogicalOperator, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            op: op,
            right: Box::new(right),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LogicalOperator {
    EqualTo,
}

impl LogicalOperator {
    pub fn from_token(token: &Token) -> Option<Self> {
        match *token {
            Token::Equals => Some(LogicalOperator::EqualTo),
            _ => None,
        }
    }
}
