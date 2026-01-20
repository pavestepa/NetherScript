use crate::ast::{ast::Ast, Ident, SyntaxError, TypedBinding};

#[derive(Debug)]
pub struct StructDecl {
    ident: Ident,
    pub fields: Ast<Vec<TypedBinding>>,
    pub syntax_errors: Vec<SyntaxError>,
}
