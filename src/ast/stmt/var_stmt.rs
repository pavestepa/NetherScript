use crate::{Atom, ast::Expr};

#[derive(Debug)]
pub struct VarStmt {
  kind: VarKind,  // let / const / var
  name: Atom,
  init: Option<Expr>,
}

#[derive(Debug)]
pub enum VarKind {
    Let,
    Const,
    Var,
}