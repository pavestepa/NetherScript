mod fn_args; pub use fn_args::FnArg;
mod fn_expr; pub use fn_expr::FnExpr;

use crate::{er::types::HasType};

pub struct FnStatmnt {
  pub is_pub: bool,
  pub name: String,
  pub args: Vec<FnArg>,
  pub return_type: HasType,
  pub body: Vec<FnExpr>,
}