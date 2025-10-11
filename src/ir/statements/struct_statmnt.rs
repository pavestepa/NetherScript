use crate::ir::declorations::struct_decl::StructField;

pub struct StructStatmnt {
  pub is_pub: bool,
  pub name: String,
  pub fields: Vec<StructField>
}