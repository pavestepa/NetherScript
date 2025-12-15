use crate::Atom;

use super::Expr;

#[derive(Debug)]
pub struct ClassConstructExpr {
  callee: Box<Atom>,
  args: Vec<Expr>,
}
