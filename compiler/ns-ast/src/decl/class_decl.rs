use crate::{Function, Ident, TypedBinding};

#[derive(Debug)]
pub struct ClassDecl {
    pub ident: Ident,
    pub extends: Option<Ident>,
    pub implements: Option<Vec<Ident>>,
    pub fields: Vec<TypedBinding>,
    pub methods: Vec<Function>,
}
