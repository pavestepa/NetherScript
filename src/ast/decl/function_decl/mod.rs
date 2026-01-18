use crate::ast::{ast::Ast, BlockStmt, Ident, TypeRef, TypedBindingPtrn};

#[derive(Debug)]
pub struct FunctionDecl {
    pub ident: Ident,
    pub args: Vec<Ast<TypedBindingPtrn>>,
    pub returns: TypeRef,
    pub body: BlockStmt,
}

impl FunctionDecl {
    pub fn new(
        ident: Ident,
        args: Vec<Ast<TypedBindingPtrn>>,
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
