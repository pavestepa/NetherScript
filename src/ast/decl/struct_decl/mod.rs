use crate::ast::{Ident, TypedBindingPtrn};

#[derive(Debug)]
pub struct StructDecl {
    ident: Ident,
    pub fields: Vec<TypedBindingPtrn>,
}
