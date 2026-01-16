use crate::ast::{BlockStmt, Ident, TypeRef, TypedBindingPtrn};

#[derive(Debug)]
pub struct FunctionDecl {
    ident: Ident,
    args: Vec<TypedBindingPtrn>,
    returns: TypeRef,
    body: BlockStmt,
}

impl FunctionDecl {
    pub fn new(
        ident: Ident,
        args: Vec<TypedBindingPtrn>,
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
