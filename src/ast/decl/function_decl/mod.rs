use crate::ast::{ast::Ast, Binding, Ident, StmtsBlock, TypeRef};

#[derive(Debug)]
pub struct FunctionDecl {
    pub ident: Ident,
    pub args: Vec<Ast<Binding>>,
    pub returns: Ast<TypeRef>,
    pub body: StmtsBlock,
}

impl FunctionDecl {
    pub fn new(
        ident: Ident,
        args: Vec<Ast<Binding>>,
        returns: Ast<TypeRef>,
        body: StmtsBlock,
    ) -> Self {
        Self {
            ident,
            args,
            returns,
            body,
        }
    }
}
