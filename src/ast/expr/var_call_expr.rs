use crate::Atom;

use super::Expr;

#[derive(Debug)]
pub struct VarCallExpr {
    pub name: Atom,
}

impl VarCallExpr {
    pub fn new(name: Atom) -> Self {
        Self { name }
    }
}
