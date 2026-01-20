use crate::ast::{ast::Ast, BlockStmt, Ident, SyntaxError, TypeRef, TypedBinding};

#[derive(Debug)]
pub struct FunctionDecl {
    pub ident: Ident,
    pub args: Vec<Ast<TypedBinding>>,
    pub returns: TypeRef,
    pub body: BlockStmt,
    pub syntax_errors: Vec<SyntaxError>,
}

impl FunctionDecl {
    pub fn new(
        ident: Ident,
        args: Vec<Ast<TypedBinding>>,
        returns: TypeRef,
        body: BlockStmt,
        syntax_errors: Vec<SyntaxError>,
    ) -> Self {
        Self {
            ident,
            args,
            returns,
            body,
            syntax_errors,
        }
    }
}
