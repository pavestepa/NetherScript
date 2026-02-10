use crate::ast::{ast::Ast, Ident, TypeRef};

#[derive(Debug, Clone)]
pub struct Binding {
    pub ident: Ident,
    pub type_ref: Ast<TypeRef>,
}
