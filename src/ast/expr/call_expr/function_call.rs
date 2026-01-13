use crate::{
    ast::{Expr, Ident},
    lexer::Token,
    Atom,
};

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub callee: Box<Ident>,
    pub args: Vec<Expr>,
}

impl FunctionCall {
    pub fn new(callee: Ident, args: Vec<Expr>) -> Self {
        Self {
            callee: Box::new(callee),
            args: args,
        }
    }
}
