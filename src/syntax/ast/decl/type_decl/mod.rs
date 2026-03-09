use crate::syntax::ast::{Ident, SyntaxError, TypeNode};

#[derive(Debug)]
pub struct TypeDecl {
    ident: Ident,
    type_value: TypeNode,
}
