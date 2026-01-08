mod fn_arg;

use crate::{
    ast::{stmt::BlockStmt, Stmt, Typ},
    Atom,
};
pub use fn_arg::FnArg;

#[derive(Debug)]
pub struct FunctionDecl {
    is_pub: bool,
    ident: Atom,
    args: Vec<FnArg>,
    returns: Typ,
    body: BlockStmt,
}

impl FunctionDecl {
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
