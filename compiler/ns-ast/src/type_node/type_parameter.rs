use crate::{type_node::TypeNode, Ident};

#[derive(Debug, Clone)]
pub struct TypeParameter {
    pub ident: Ident,
    pub constaint: Option<TypeNode>,
    pub default_type: Option<TypeNode>,
}
