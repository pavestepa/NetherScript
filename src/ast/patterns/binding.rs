use crate::ast::{Ident, TypeRef};

#[derive(Debug, Clone)]
pub struct Binding {
    pub ident: Ident,
    pub type_ref: TypeRef,
}
