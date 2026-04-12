use crate::{
    {ast::Ast, Expr, Ident, TypeNode},

};

#[derive(Debug)]
pub struct ConstDecl {
    pub ident: Ident,
    pub data_type: TypeNode,
    pub val: Ast<Expr>,
}

impl ConstDecl {
    pub fn new(
        ident: Ident,
        data_type: TypeNode,
        val: Ast<Expr>
    ) -> Self {
        Self {
            ident,
            data_type,
            val,
        }
    }
}
