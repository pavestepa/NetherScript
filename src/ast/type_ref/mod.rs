mod ref_kind;
mod type_ident;

use crate::ast::{ident, shared::RefKind, Ident};

#[derive(Debug, Clone)]
pub struct TypeRef {
    ref_kind: RefKind,
    ident: Ident,
    generics: Option<Vec<TypeRef>>,
}

impl TypeRef {
    pub fn new_without_generic(ref_kind: RefKind, ident: Ident) -> Self {
        Self {
            ref_kind,
            ident,
            generics: None,
        }
    }
}
