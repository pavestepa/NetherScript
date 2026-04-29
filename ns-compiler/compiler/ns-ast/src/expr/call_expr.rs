use crate::{Expr, TypeNode};

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub type_arguments: Vec<TypeNode>,
    pub args: Vec<Expr>,
}

impl CallExpr {
    pub fn new(callee: Box<Expr>, type_arguments: Vec<TypeNode>, args: Vec<Expr>) -> Self {
        Self {
            callee: callee,
            type_arguments,
            args: args,
        }
    }
}
