use crate::semantics::hir::{exprs::Expr, stmts::Stmts};

pub struct IfStmt {
    pub test: Box<Expr>,
    pub body: Box<Stmts>,
    pub alt: Option<Box<Stmts>>,
}
