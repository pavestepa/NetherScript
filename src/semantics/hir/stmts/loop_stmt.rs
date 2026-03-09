use crate::semantics::hir::stmts::Stmts;

pub struct LoopStmt {
    pub body: Box<Stmts>,
}
