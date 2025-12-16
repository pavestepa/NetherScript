use crate::Atom;

use super::Expr;

#[derive(Debug)]
pub struct FnCallExpr {
  pub name: Atom,
  pub args: Vec<Expr>
}

impl FnCallExpr {
  pub fn new(name: Atom, args: Vec<Expr>) -> Self {
    Self {
      name,
      args
    }
  } 
}