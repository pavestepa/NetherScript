use crate::syntax::ast::{ast::Ast, Expr, Ident};

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub callee: Ident,
    pub args: Vec<Ast<Expr>>,
}

impl FunctionCall {
    pub fn new(callee: Ident, args: Vec<Ast<Expr>>) -> Self {
        Self {
            callee: callee,
            args: args,
        }
    }
}
