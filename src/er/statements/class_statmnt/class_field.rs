use crate::er::{assign::AssignCall, types::HasType};

pub struct ClassField {
  pub is_pub: bool,
  pub is_readonly: bool,
  pub name: String,
  pub has_type: Option<HasType>,
  pub assign: Option<AssignCall>
}