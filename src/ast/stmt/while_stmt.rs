use crate::ast::{stmt::BlockStmt, Expr};

#[derive(Debug)]
pub struct WhileStmt {
    pub test: Box<Expr>,
    pub body: Box<BlockStmt>,
}

impl WhileStmt {
    pub fn new(test: Expr, body: BlockStmt) -> Self {
        Self {
            test: Box::new(test),
            body: Box::new(body),
        }
    }
}
