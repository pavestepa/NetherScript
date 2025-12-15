pub mod fn_arg;

use crate::{Atom, ast::{Stmt, Typ}};
use fn_arg::FnArg;

#[derive(Debug)]
pub struct FnDecl {
  is_pub: bool,
  ident: Atom,
  args: Vec<FnArg>,
  returns: Typ,
  body: Vec<Stmt>
}

impl FnDecl {
  pub fn new(is_pub: bool, ident: Atom, args: Vec<FnArg>, returns: Typ, body: Vec<Stmt>) -> Self {
    Self { is_pub, ident, args, returns, body }
  }
}