use crate::syntax::ast::{Binding, Ident, RefKind, StmtsBlock, TypeNode, ast::Ast, shared::this::This};

#[derive(Debug)]
pub struct Method {
    pub is_pub: bool,
    pub ident: Ident,
    //* first option is for static, second for refkind or not */
    pub this: This,
    pub args: Vec<Ast<Binding>>,
    pub returns: Ast<TypeNode>,
    pub body: StmtsBlock,
}

impl Method {
    pub fn new(
        is_pub: bool,
        ident: Ident,
        this: This,
        args: Vec<Ast<Binding>>,
        returns: Ast<TypeNode>,
        body: StmtsBlock,
    ) -> Self {
        Self {
            is_pub,
            ident,
            this,
            args,
            returns,
            body,
        }
    }
}
