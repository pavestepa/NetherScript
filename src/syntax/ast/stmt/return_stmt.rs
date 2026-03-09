use crate::syntax::ast::{ast::Ast, Expr};

#[derive(Debug)]
pub struct ReturnStmt {
    pub arg: Option<Box<Ast<Expr>>>,
}

impl ReturnStmt {
    pub fn new(arg: Option<Ast<Expr>>) -> Self {
        Self {
            arg: match arg {
                Some(e) => Some(Box::new(e)),
                None => None,
            },
        }
    }
}
