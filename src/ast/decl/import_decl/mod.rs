use crate::ast::{ast::Ast, Ident};

#[derive(Debug)]
pub struct ImportDecl {
    pub ident: Ast<Ident>,
    pub from: Ast<Vec<Ident>>,
}

impl ImportDecl {
    pub fn new(ident: Ast<Ident>, from: Ast<Vec<Ident>>) -> Self {
        Self { ident, from }
    }
}
