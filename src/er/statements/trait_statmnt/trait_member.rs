use crate::er::{statements::{fn_statmnt::FnArg, trait_statmnt::trait_default_impl::TraitDefaultImpl}, types::HasType};

pub struct TraitMember {
  name: String,
  args: Vec<FnArg>,
  return_type: HasType,
  default_impl: Option<TraitDefaultImpl>
}