use crate::{ast::Ast, Ident, Method, TypedBinding};

#[derive(Debug)]
pub struct ClassDecl {
    pub ident: Ident,
    pub extends: Option<Ident>,
    pub implements: Option<Vec<Ident>>,
    pub fields: Vec<Ast<TypedBinding>>,
    pub methods: Vec<Ast<Method>>,
}
