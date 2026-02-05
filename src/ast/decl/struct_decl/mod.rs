use crate::ast::{ast::Ast, Binding, Ident, SyntaxError};

#[derive(Debug)]
pub struct StructDecl {
    ident: Ident,
    pub fields: Ast<Vec<Binding>>,
    pub syntax_errors: Vec<SyntaxError>,
}
