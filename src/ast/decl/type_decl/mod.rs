use crate::ast::Ident;

#[derive(Debug)]
pub struct TypeDecl {
    ident: Ident,
    type_value: TypeRef,
}
