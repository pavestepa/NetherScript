use crate::lexer::Token;

use super::Expr;

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    left: Box<Expr>,
    op: BinaryOperator,
    right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, op: BinaryOperator, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            op: op,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equals,
    NotEquals,
    BitAnd,
    BitXor,
    BitOr,
    And,
    Or,
    ShiftLeft,
    ShiftRight,
}

impl BinaryOperator {
    pub fn from_token(token: &Token) -> Option<Self> {
        match *token {
            Token::Plus => Some(BinaryOperator::Plus),
            Token::Minus => Some(BinaryOperator::Minus),
            Token::Star => Some(BinaryOperator::Star),
            Token::Slash => Some(BinaryOperator::Slash),
            Token::Percent => Some(BinaryOperator::Percent),
            Token::Greater => Some(BinaryOperator::Greater),
            Token::GreaterEqual => Some(BinaryOperator::GreaterEqual),
            Token::Less => Some(BinaryOperator::Less),
            Token::LessEqual => Some(BinaryOperator::LessEqual),
            Token::Equals => Some(BinaryOperator::Equals),
            Token::NotEquals => Some(BinaryOperator::NotEquals),
            Token::BitAnd => Some(BinaryOperator::BitAnd),
            Token::BitXor => Some(BinaryOperator::BitXor),
            Token::BitOr => Some(BinaryOperator::BitOr),
            Token::And => Some(BinaryOperator::And),
            Token::Or => Some(BinaryOperator::Or),
            Token::ShiftLeft => Some(BinaryOperator::ShiftLeft),
            Token::ShiftRight => Some(BinaryOperator::ShiftRight),
            _ => None,
        }
    }
}
