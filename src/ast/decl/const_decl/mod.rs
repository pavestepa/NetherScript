use crate::ast::{ast::Ast, Expr, Ident, SyntaxError, TypeRef};

#[derive(Debug)]
pub struct ConstDecl {
    pub ident: Ident,
    pub data_type: TypeRef,
    pub val: Ast<Expr>,
    pub syntax_errors: Vec<SyntaxError>,
}

impl ConstDecl {
    pub fn new(
        ident: Ident,
        data_type: TypeRef,
        val: Ast<Expr>,
        syntax_errors: Vec<SyntaxError>,
    ) -> Self {
        Self {
            ident,
            data_type,
            val,
            syntax_errors,
        }
    }
}
