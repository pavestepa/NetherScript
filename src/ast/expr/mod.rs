pub mod call_expr;
pub mod op_expr;
pub mod unary_expr;

pub use call_expr::{BindignCall, CallExpr, FunctionCall, LiteralCall, MemberCall};
pub use op_expr::{BinaryOp, LogicalOp, Op, OpExpr};
pub use unary_expr::UnaryExpr;

use crate::ast::RefKind;
#[derive(Debug, Clone)]
pub struct Expr {
    expr_kind: ExprKind,
    ref_kind: RefKind,
    scoped: bool,
}
#[derive(Debug, Clone)]
pub enum ExprKind {
    Call(CallExpr),
    Op(OpExpr),
    UnaryExpr(UnaryExpr),
}
