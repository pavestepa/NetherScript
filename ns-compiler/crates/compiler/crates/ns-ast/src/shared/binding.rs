use crate::{Ident, TypeNode};

#[derive(Debug, Clone)]
pub struct Binding {
    pub ident: Ident,
    pub type_ref: Option<TypeNode>,
}

#[derive(Debug, Clone)]
pub struct TypedBinding {
    pub ident: Ident,
    pub type_ref: TypeNode,
}
