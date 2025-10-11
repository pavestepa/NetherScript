mod arithmetic_expr; pub use arithmetic_expr::{ArithmeticExpr, Arithmetic};
mod fn_call_expr; pub use fn_call_expr::FnCallExpr;
mod var_call_expr; pub use var_call_expr::VarCallExpr;
mod value_expr; pub use value_expr::ValueExpr;
mod object_expr; pub use object_expr::ObjectExpr;

pub enum Expr {
  Arithmetic(Arithmetic),
  FnCallExpr(FnCallExpr),
  VarCallExpr(VarCallExpr),
  ValueExpr(ValueExpr),
  ObjectExpr(ObjectExpr)
}
