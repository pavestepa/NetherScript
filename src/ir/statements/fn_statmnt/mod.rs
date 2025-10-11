use crate::ir::HasType;

pub mod fn_arg; use fn_arg::FnArg;
pub mod fn_expr; use fn_expr::FnExpr;

pub struct FnStatmnt {
  pub is_pub: bool,
  pub name: String,
  pub args: Vec<FnArg>,
  pub return_type: HasType,
  pub body: Vec<FnExpr>,
}