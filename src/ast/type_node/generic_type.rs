use crate::ast::{type_node::TypeNode, Ident};

#[derive(Debug, Clone)]
pub struct GenericType {
    pub ident: Ident,
    pub arguments: Option<Vec<TypeNode>>,
}
impl GenericType {
    pub fn new_without_generic(ident: Ident, arguments: Option<Vec<TypeNode>>) -> Self {
        Self { ident, arguments }
    }
}
