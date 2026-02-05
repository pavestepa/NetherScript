mod binary_op;
mod logical_op;
mod unary_op;

pub use binary_op::BinaryOp;
pub use logical_op::LogicalOp;
pub use unary_op::UnaryOp;

#[derive(Debug, Clone)]
pub enum OpExpr {
    Binary(BinaryOp),
    Logical(LogicalOp),
    Unary(UnaryOp),
    Error,
}
