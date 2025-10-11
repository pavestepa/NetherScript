pub mod arithmetic_expr; pub use arithmetic_expr::Arithmetic;
pub mod fn_call_expr; pub use fn_call_expr::FnCallExpr;

use crate::ir::ObjectExpr;
pub enum Expr {
  Arithmetic(Arithmetic),
  FnCallExpr(FnCallExpr),
  VarCallExpr,
  ValueExpr,
  ObjectExpr(ObjectExpr),
}