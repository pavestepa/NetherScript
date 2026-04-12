use crate::{ast::Ast, EnumMember, Ident};

#[derive(Debug)]
pub struct EnumDecl {
    ident: Ident,
    members: Vec<Ast<EnumMember>>,
}
