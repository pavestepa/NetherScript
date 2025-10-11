use crate::er::{statements::fn_statmnt::{FnArg, FnExpr}, types::HasType};

pub struct ClassMethod {
  pub is_pub: bool,
  pub is_static: bool,
  pub name: String,
  pub args: Vec<FnArg>,
  pub return_type: HasType,
  pub body: Vec<FnExpr>
}