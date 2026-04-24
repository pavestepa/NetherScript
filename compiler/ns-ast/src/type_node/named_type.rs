use crate::{Ident, type_node::TypeNode};

/// TypeScript-style type reference: `T`, `number`, or `Promise<string>` with type arguments.
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
