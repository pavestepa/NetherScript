use crate::ast::{ast::Ast, Expr, Ident, TypeRef};

#[derive(Debug)]
pub struct ConstDecl {
    ident: Ident,
    data_type: TypeRef,
    val: Ast<Expr>,
}

impl ConstDecl {
    pub fn new(ident: Ident, data_type: TypeRef, val: Ast<Expr>) -> Self {
        Self {
            ident,
            data_type,
            val,
        }
    }
}
