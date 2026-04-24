use crate::{Expr};

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

impl CallExpr {
    pub fn new(callee: Box<Expr>, args: Vec<Expr>) -> Self {
        Self {
            callee: callee,
            args: args,
        }
    }
}
