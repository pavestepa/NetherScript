use crate::ast::{ast::Ast, EnumMember, Ident, SyntaxError};

#[derive(Debug)]
pub struct EnumDecl {
    ident: Ident,
    members: Vec<Ast<EnumMember>>,
    syntax_errors: Vec<SyntaxError>,
}
