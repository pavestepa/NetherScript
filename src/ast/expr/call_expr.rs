use crate::{lexer::Token, Atom};

use super::Expr;

// TODO: think more
#[derive(Debug)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub ident: Atom,
    pub args: Option<Vec<Expr>>,
    pub prop: Option<Box<CallExpr>>,
}

impl CallExpr {
    pub fn new(callee: Expr, ident: Atom, args: Option<Vec<Expr>>, prop: Option<CallExpr>) -> Self {
        Self {
            callee: Box::new(callee),
            ident,
            args: args,
            prop: match prop {
                Some(v) => Some(Box::new(v)),
                None => None,
            },
        }
    }
}
