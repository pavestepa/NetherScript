pub struct ScopeNode {
  id: usize,
  kind: String, //func, block, loop
  parent: Box<ScopeNode>,
  variables: Vec<String>,
  return_lifetimes: String, // for functions
}