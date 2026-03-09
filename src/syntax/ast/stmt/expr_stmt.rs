use crate::syntax::ast::{ast::Ast, Expr};

#[derive(Debug)]
pub struct ExprStmt {
    pub expr: Box<Ast<Expr>>,
}

impl ExprStmt {
    pub fn new(expr: Ast<Expr>) -> Self {
        Self {
            expr: Box::new(expr),
        }
    }
}
