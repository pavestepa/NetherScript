use crate::{
    ast::{shared::VarKind, Expr},
    Atom,
};

#[derive(Debug)]
pub struct VarStmt {
    pub kind: VarKind,
    pub declare: bool,
    pub name: Atom,
    pub init: Option<Box<Expr>>,
}

impl VarStmt {
    pub fn new(kind: VarKind, name: Atom, init: Option<Box<Expr>>) -> Self {
        Self {
            kind: kind,
            declare: init.is_some(),
            name: name,
            init: init,
        }
    }
}
