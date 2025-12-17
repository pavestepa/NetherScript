use crate::ast::Expr;

#[derive(Debug)]
pub struct ExprStmt {
    pub expr: Box<Expr>,
}
