use crate::{lexer::Token, Atom};

use super::Expr;

// TODO: think more
#[derive(Debug)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

impl CallExpr {
    pub fn new(callee: Expr, args: Vec<Expr>) -> Self {
        Self {
            callee: Box::new(callee),
            args: args,
        }
    }
}
