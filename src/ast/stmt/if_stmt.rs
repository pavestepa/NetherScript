use crate::ast::{stmt::BlockStmt, Expr, Stmt};

#[derive(Debug)]
pub struct IfStmt {
    pub test: Box<Expr>,
    pub body: Box<BlockStmt>,
    pub alt: Option<Box<BlockStmt>>,
}

impl IfStmt {
    pub fn new(test: Expr, body: BlockStmt, alt: Option<BlockStmt>) -> Self {
        let alt_stmt = match alt {
            Some(v) => Some(Box::new(v)),
            None => None,
        };
        Self {
            test: Box::new(test),
            body: Box::new(body),
            alt: alt_stmt,
        }
    }
}
