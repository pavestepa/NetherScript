use crate::{ast::Expr, Atom};

#[derive(Debug)]
pub struct PropertyExpr {
    pub object: Box<Expr>,
    pub name: Atom,
}

impl PropertyExpr {
    pub fn new(object: Expr, name: Atom) -> Self {
        Self {
            object: Box::new(object),
            name,
        }
    }
}
