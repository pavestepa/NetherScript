mod binary_op;
mod logical_op;

pub use binary_op::BinaryOp;
pub use logical_op::LogicalOp;

use crate::ast::Expr;

#[derive(Debug, Clone)]
pub struct OpExpr {
    left: Box<Expr>,
    operate: Op,
}

#[derive(Debug, Clone)]
pub enum Op {
    Binary(BinaryOp),
    Logical(LogicalOp),
    Error,
}
