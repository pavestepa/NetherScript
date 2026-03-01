use crate::ast::Expr;

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
    left: Box<Expr>,
    kind: BinaryOperator,
    right: Box<Expr>,
}
