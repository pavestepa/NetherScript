mod type_ident;

use crate::ast::{ident, Ident};

#[derive(Debug, Clone)]
pub struct TypeRef {
    pub ident: Ident,
    pub generics: Option<Vec<TypeRef>>,
}

impl TypeRef {
    pub fn new(ident: Ident) -> Self {
        Self {
            ident,
            generics: None,
        }
    }
}
