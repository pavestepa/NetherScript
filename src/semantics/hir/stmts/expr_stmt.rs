use crate::semantics::hir::exprs::Expr;

pub struct ExprStmt {
    pub expr: Box<Expr>,
}
