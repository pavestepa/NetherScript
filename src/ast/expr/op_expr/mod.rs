mod binary_op;
mod logical_op;
mod unary_op;

pub use binary_op::BinaryOp;
pub use logical_op::LogicalOp;
pub use unary_op::UnaryOp;

use crate::ast::Expr;

#[derive(Debug, Clone)]
pub struct OpExpr {
    left: Box<Expr>,
    operate: OpKind,
    scoped: bool,
}

#[derive(Debug, Clone)]
pub enum OpKind {
    Binary(BinaryOp),
    Logical(LogicalOp),
    Unary(UnaryOp),
    Error,
}
