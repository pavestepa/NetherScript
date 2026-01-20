use crate::ast::{ast::Ast, BlockStmt, Ident, SyntaxError, TypeRef, TypedBinding};

#[derive(Debug)]
pub struct FunctionDecl {
    pub ident: Ident,
    pub args: Vec<Ast<TypedBinding>>,
    pub returns: TypeRef,
    pub body: BlockStmt,
}

impl FunctionDecl {
    pub fn new(
        ident: Ident,
        args: Vec<Ast<TypedBinding>>,
        returns: TypeRef,
        body: BlockStmt,
    ) -> Self {
        Self {
            ident,
            args,
            returns,
            body,
        }
    }
}
