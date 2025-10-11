use crate::ir::types::HasType;

pub struct FnArg {
  pub name: String,
  pub has_type: HasType,
}