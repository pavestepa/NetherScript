use crate::syntax::ast::{ast::Ast, Ident, TypeNode};

#[derive(Debug, Clone)]
pub struct Binding {
    pub ident: Ident,
    pub type_ref: Option<Ast<TypeNode>>,
}
