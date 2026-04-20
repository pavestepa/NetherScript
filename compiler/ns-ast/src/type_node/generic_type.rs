use crate::{Ident, type_node::TypeNode};

#[derive(Debug, Clone)]
pub struct GenericType {
    pub ident: Ident,
    pub arguments: Option<Vec<TypeNode>>,
}
impl GenericType {
    pub fn new(ident: Ident, arguments: Option<Vec<TypeNode>>) -> Self {
        Self { ident, arguments }
    }
}
