use crate::ast::Expr;

#[derive(Debug)]
pub struct ExprStmt {
    pub expr: Box<Expr>,
}

impl ExprStmt {
    pub fn new(expr: Expr) -> Self {
        Self {
            expr: Box::new(expr),
        }
    }
}
