use crate::{lexer::Token, Atom};

use super::Expr;

// TODO: think more
#[derive(Debug)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

impl CallExpr {
    pub fn new(callee: Expr, ident: Atom, args: Vec<Expr>, prop: Option<CallExpr>) -> Self {
        Self {
            callee: Box::new(callee),
            args: args,
        }
    }
}
