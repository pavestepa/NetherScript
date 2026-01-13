mod assign_op;
mod binary_op;
mod logical_op;
mod unary_op;

pub use assign_op::AssignOp;
pub use binary_op::BinaryOp;
pub use logical_op::LogicalOp;
pub use unary_op::UnaryOp;

#[derive(Debug, Clone)]
pub enum OpExpr {
    Assign(AssignOp),
    Binary(BinaryOp),
    Logical(LogicalOp),
    Unary(UnaryOp),
}
