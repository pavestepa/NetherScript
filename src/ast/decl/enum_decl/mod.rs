use crate::ast::{patterns::EnumMember, Ident};

#[derive(Debug)]
pub struct EnumDecl {
    ident: Ident,
    members: Vec<Ast<EnumMember>>,
}
