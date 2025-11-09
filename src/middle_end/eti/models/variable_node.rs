use crate::middle_end::eti::models::{lifetime_node::LifetimeNode, scope_node::ScopeNode};

pub struct VariableNode {
  id: usize,
  name: String,
  declared_type: String,
  inferred_type: String,
  mode: String,
  lifetime: Box<LifetimeNode>,
  scope: Box<ScopeNode>,
  flags: Vec<String>,
}