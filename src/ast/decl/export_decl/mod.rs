use crate::ast::{ast::Ast, Ident};

#[derive(Debug)]
pub struct ExportDecl {
    pub ident: Ast<Ident>,
}

impl ExportDecl {
    pub fn new(ident: Ast<Ident>) -> Self {
        Self { ident }
    }
}
