use crate::{Atom, ast::{Expr, Typ}};

#[derive(Debug)]
pub struct GlobalConstDecl {
    is_pub: bool,
    ident: Atom,
    typ: Typ,
    val: Expr
}

impl GlobalConstDecl {
    pub fn new(is_pub: bool, ident: Atom, typ: Typ, val: Expr) -> Self {
        Self {
            is_pub,
            ident,
            typ,
            val
        }
    }
}
