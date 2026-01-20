use crate::ast::{Ident, SyntaxError, TypeRef};

#[derive(Debug)]
pub struct TypeDecl {
    ident: Ident,
    type_value: TypeRef,
    pub syntax_errors: Vec<SyntaxError>,
}
