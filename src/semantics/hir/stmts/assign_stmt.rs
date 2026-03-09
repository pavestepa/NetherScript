use crate::{semantics::hir::exprs::Expr, Atom};

pub struct AssignStmt {
    pub id: Atom,
    pub assign: Box<Expr>,
}
