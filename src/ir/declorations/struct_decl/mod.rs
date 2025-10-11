use crate::ir::types::HasType;

pub struct StructField {
  pub is_pub: bool,
  pub name: String,
  pub has_type: HasType
}