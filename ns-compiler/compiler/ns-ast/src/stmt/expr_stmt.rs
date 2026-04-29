use crate::Expr;

#[derive(Debug, Clone)]
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
