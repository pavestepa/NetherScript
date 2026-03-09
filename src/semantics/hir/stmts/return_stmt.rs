use crate::semantics::hir::exprs::Expr;

pub struct ReturnStmt {
    pub arg: Option<Box<Expr>>,
}
