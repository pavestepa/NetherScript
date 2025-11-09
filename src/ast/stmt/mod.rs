use crate::{Atom, ast::{Expr, Typ}};

#[derive(Debug)]
pub enum Stmt {
    VarDecl {
        kind: VarKind,  // let / const / var
        name: Atom,
        init: Option<Expr>,
    },
    ExprStmt(Expr), // например: foo()
}

#[derive(Debug)]
pub enum VarKind {
    Let,
    Const,
    Var,
}

#[derive(Debug)]
pub struct ArgStmt {
    ident: Atom,
    type_kind: Typ,
}

impl ArgStmt {
    pub fn new(ident: Atom, type_kind: Typ) -> Self {
        Self { ident, type_kind }
    }
}