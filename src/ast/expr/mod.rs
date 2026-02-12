pub mod call_expr;
pub mod op_expr;

pub use call_expr::{BindignCall, CallExpr, FunctionCall, LiteralCall, MemberCall};
pub use op_expr::{BinaryOp, LogicalOp, OpExpr, UnaryOp};

#[derive(Debug, Clone)]
pub enum Expr {
    Call(CallExpr),
    Op(OpExpr),
    Error,
}
