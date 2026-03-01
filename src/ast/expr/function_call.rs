use crate::ast::{Expr, Ident};

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub callee: Ident,
    pub args: Vec<Expr>,
}

impl FunctionCall {
    pub fn new(callee: Ident, args: Vec<Expr>) -> Self {
        Self {
            callee: callee,
            args: args,
        }
    }
}
