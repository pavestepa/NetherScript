mod ref_kind;
mod type_ident;

use crate::ast::{shared::RefKind, Ident};

#[derive(Debug, Clone)]
pub struct TypeRef {
    ref_kind: RefKind,
    ident: Ident,
    generics: Option<Vec<TypeRef>>,
}
