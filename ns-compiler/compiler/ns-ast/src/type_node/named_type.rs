use crate::{Ident, type_node::TypeNode};

/// Refers to a type by name and optionally supplies type arguments in declaration order.
#[derive(Debug, Clone)]
pub struct NamedType {
    pub ident: Ident,
    pub type_arguments: Vec<TypeNode>,
}

impl NamedType {
    pub fn new(ident: Ident, type_arguments: Vec<TypeNode>) -> Self {
        Self {
            ident,
            type_arguments,
        }
    }

    pub fn simple(ident: Ident) -> Self {
        Self {
            ident,
            type_arguments: Vec::new(),
        }
    }
}
