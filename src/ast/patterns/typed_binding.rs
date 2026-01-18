use crate::ast::{Ident, TypeRef};

#[derive(Debug, Clone)]
pub struct TypedBinding {
    pub ident: Ident,
    pub type_ref: TypeRef,
}
