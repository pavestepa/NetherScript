use crate::ast::Ident;

#[derive(Debug)]
pub struct ImportDecl {
    ident: Ast<Ident>,
    pub from: Ast<Vec<Ident>>,
}
