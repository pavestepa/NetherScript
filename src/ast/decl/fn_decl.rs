use crate::{Atom, ast::{Stmt, Typ, stmt::ArgStmt}};

#[derive(Debug)]
pub struct FnDecl {
  is_pub: bool,
  ident: Atom,
  args: Vec<ArgStmt>,
  returns: Typ,
  body: Vec<Stmt>
}

impl FnDecl {
  pub fn new(is_pub: bool, ident: Atom, args: Vec<ArgStmt>, returns: Typ, body: Vec<Stmt>) -> Self {
    Self { is_pub, ident, args, returns, body }
  }
}
