use crate::{
    ast::{Expr, TypeRef},
    Atom,
};

#[derive(Debug)]
pub struct ConstDecl {
    is_pub: bool,
    ident: Atom,
    data_type: TypeRef,
    val: Expr,
}

impl ConstDecl {
    pub fn new(is_pub: bool, ident: Atom, data_type: TypeRef, val: Expr) -> Self {
        Self {
            is_pub,
            ident,
            data_type,
            val,
        }
    }
}
