pub mod call_expr;
pub mod op_expr;

pub use call_expr::CallExpr;
pub use op_expr::OpExpr;

#[derive(Debug, Clone)]
pub enum Expr {
    Call(CallExpr),
    Op(OpExpr),
    Error,
}
