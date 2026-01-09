use crate::ast::Expr;

#[derive(Debug, Clone)]
pub struct IndexExpr {
    target: Box<Expr>,
    index: Box<Expr>,
}

impl IndexExpr {
    pub fn new(target: Expr, index: Expr) -> Self {
        Self {
            target: Box::new(target),
            index: Box::new(index),
        }
    }
}
