use crate::ast::Ident;

#[derive(Debug)]
pub struct ImportDecl {
    ident: Ident,
    pub from: Vec<Ident>,
}
