use crate::{Atom, ast::Expr};

#[derive(Debug)]
pub struct VarStmt {
  kind: VarKind,  // let / const / var
  name: Atom,
  init: Option<Expr>,
}

impl VarStmt {
  pub fn new(kind: VarKind, name: Atom, init: Option<Expr>) -> Self {
    Self {
      kind,
      name,
      init
    }
  } 
}

#[derive(Debug)]
pub enum VarKind {
    Let,
    Const,
    Var,
}