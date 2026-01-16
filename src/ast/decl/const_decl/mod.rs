use crate::ast::{Expr, Ident, TypeRef};

#[derive(Debug)]
pub struct ConstDecl {
    ident: Ident,
    data_type: TypeRef,
    val: Expr,
}

impl ConstDecl {
    pub fn new(ident: Ident, data_type: TypeRef, val: Expr) -> Self {
        Self {
            ident,
            data_type,
            val,
        }
    }
}
