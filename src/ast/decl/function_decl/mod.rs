use crate::{
    ast::{BlockStmt, TypeRef, TypedBindingPtrn},
    Atom,
};

#[derive(Debug)]
pub struct FunctionDecl {
    export: bool,
    ident: Atom,
    args: Vec<TypedBindingPtrn>,
    returns: TypeRef,
    body: BlockStmt,
}

impl FunctionDecl {
    pub fn new(
        export: bool,
        ident: Atom,
        args: Vec<TypedBindingPtrn>,
        returns: TypeRef,
        body: BlockStmt,
    ) -> Self {
        Self {
            export,
            ident,
            args,
            returns,
            body,
        }
    }
}
