use crate::syntax::ast::{ast::Ast, Binding, Ident, StmtsBlock, TypeNode};

#[derive(Debug)]
pub struct FunctionDecl {
    pub ident: Ident,
    pub args: Vec<Ast<Binding>>,
    pub returns: Ast<TypeNode>,
    pub body: StmtsBlock,
}

impl FunctionDecl {
    pub fn new(
        ident: Ident,
        args: Vec<Ast<Binding>>,
        returns: Ast<TypeNode>,
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
