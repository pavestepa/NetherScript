use crate::{Expr, Ident, MemberExpr};

#[derive(Debug, Clone)]
pub enum AssignTarget {
    Ident(Ident),
    Member(MemberExpr),
}

#[derive(Debug, Clone)]
pub struct AssignStmt {
    pub target: AssignTarget,
    pub assign: Box<Expr>,
}
