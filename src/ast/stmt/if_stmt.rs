use crate::ast::{Expr, Stmt};

#[derive(Debug)]
pub struct IfStmt {
    pub test: Box<Expr>,
    pub body: Box<Stmt>,
    pub alt: Option<Box<Stmt>>,
}
