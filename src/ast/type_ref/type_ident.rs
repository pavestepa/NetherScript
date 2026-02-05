use crate::ast::Ident;

pub struct TypeIdent {
    ident: Ident,
    generics: Option<Vec<TypeIdent>>,
}
