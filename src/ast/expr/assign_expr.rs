use crate::lexer::Token;

use super::Expr;

#[derive(Debug)]
pub struct AssignExpr {
    left: Box<Expr>,
    op: AssignOperator,
    right: Box<Expr>,
}

impl AssignExpr {
    pub fn new(left: Expr, op: AssignOperator, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            op: op,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub enum AssignOperator {
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
}

impl AssignOperator {
    pub fn from_token(token: &Token) -> Option<Self> {
        match *token {
            Token::Assign => Some(AssignOperator::Assign),
            Token::PlusAssign => Some(AssignOperator::PlusAssign),
            Token::MinusAssign => Some(AssignOperator::MinusAssign),
            Token::StarAssign => Some(AssignOperator::StarAssign),
            Token::SlashAssign => Some(AssignOperator::SlashAssign),
            Token::PercentAssign => Some(AssignOperator::PercentAssign),
            _ => None,
        }
    }
}
