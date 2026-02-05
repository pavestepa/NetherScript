use crate::ast::{ast::Ast, Binding, Ident, StmtsBlock, SyntaxError, TypeRef};

#[derive(Debug)]
pub struct FunctionDecl {
    pub ident: Ident,
    pub args: Vec<Ast<Binding>>,
    pub returns: TypeRef,
    pub body: StmtsBlock,
}

impl FunctionDecl {
    pub fn new(ident: Ident, args: Vec<Ast<Binding>>, returns: TypeRef, body: StmtsBlock) -> Self {
        Self {
            ident,
            args,
            returns,
            body,
        }
    }
}
