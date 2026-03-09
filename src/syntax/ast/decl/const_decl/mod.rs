use crate::syntax::ast::{ast::Ast, Expr, Ident, SyntaxError, TypeNode};

#[derive(Debug)]
pub struct ConstDecl {
    pub ident: Ident,
    pub data_type: TypeNode,
    pub val: Ast<Expr>,
    pub syntax_errors: Vec<SyntaxError>,
}

impl ConstDecl {
    pub fn new(
        ident: Ident,
        data_type: TypeNode,
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
