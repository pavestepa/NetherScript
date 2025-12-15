use crate::Atom;

use super::Expr;

#[derive(Debug)]
pub struct CallFnExpr {
  callee: Box<Atom>,
  args: Vec<Expr>,
}
