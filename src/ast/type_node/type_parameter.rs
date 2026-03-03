use crate::ast::{type_node::TypeNode, Ident};

#[derive(Debug, Clone)]
pub struct TypeParameter {
    ident: Ident,
    constaint: Option<TypeNode>,
    default_type: Option<TypeNode>,
}
