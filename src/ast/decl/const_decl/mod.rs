use crate::{
    ast::{Expr, Typ},
    Atom,
};

#[derive(Debug)]
pub struct ConstDecl {
    is_pub: bool,
    ident: Atom,
    typ: Typ,
    val: Expr,
}

impl ConstDecl {
    pub fn new(is_pub: bool, ident: Atom, typ: Typ, val: Expr) -> Self {
        Self {
            is_pub,
            ident,
            typ,
            val,
        }
    }
}
