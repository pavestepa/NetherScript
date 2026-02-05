use crate::ast::{stmt::StmtsBlock, Expr};

#[derive(Debug)]
pub struct IfStmt {
    pub test: Box<Expr>,
    pub body: Box<StmtsBlock>,
    pub alt: Option<Box<StmtsBlock>>,
}

impl IfStmt {
    pub fn new(test: Expr, body: StmtsBlock, alt: Option<StmtsBlock>) -> Self {
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
