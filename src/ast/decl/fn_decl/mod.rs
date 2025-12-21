pub mod fn_arg;

use crate::{
    ast::{stmt::BlockStmt, Stmt, Typ},
    Atom,
};
use fn_arg::FnArg;

#[derive(Debug)]
pub struct FnDecl {
    is_pub: bool,
    ident: Atom,
    args: Vec<FnArg>,
    returns: Typ,
    body: BlockStmt,
}

impl FnDecl {
    pub fn new(is_pub: bool, ident: Atom, args: Vec<FnArg>, returns: Typ, body: BlockStmt) -> Self {
        Self {
            is_pub,
            ident,
            args,
            returns,
            body,
        }
    }
}
