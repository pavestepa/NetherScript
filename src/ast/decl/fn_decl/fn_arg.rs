use crate::{Atom, ast::Typ};

#[derive(Debug)]
pub struct FnArg {
    ident: Atom,
    type_kind: Typ,
}

impl FnArg {
    pub fn new(ident: Atom, type_kind: Typ) -> Self {
        Self { ident, type_kind }
    }
}