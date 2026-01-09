use crate::Atom;

use super::Expr;

#[derive(Debug, Clone)]
pub struct ClassConstructExpr {
    callee: Box<Atom>,
    args: Vec<Expr>,
}
