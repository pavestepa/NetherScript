use crate::Expr;

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
}
#[derive(Debug, Clone)]
pub struct BinaryOp {
    pub left: Box<Expr>,
    pub kind: BinaryOperator,
    pub right: Box<Expr>,
}
