
use crate::ir::{expressions::Expr, types::HasType};

pub struct VarDecl {
  pub is_mut: bool,
  pub name: String,
  pub has_type: HasType,
  pub value: Expr
}