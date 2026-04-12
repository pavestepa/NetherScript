use crate::{ast::Ast, Expr};

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
    pub left: Box<Ast<Expr>>,
    pub kind: BinaryOperator,
    pub right: Box<Ast<Expr>>,
}
