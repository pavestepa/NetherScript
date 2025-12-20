use crate::ast::Expr;

#[derive(Debug)]
pub struct TernaryExpr {
    cond: Box<Expr>,
    then_branch: Box<Expr>,
    else_branch: Box<Expr>,
}

impl TernaryExpr {
    pub fn new(cond: Expr, then_branch: Expr, else_branch: Expr) -> Self {
        Self {
            cond: Box::new(cond),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        }
    }
}
