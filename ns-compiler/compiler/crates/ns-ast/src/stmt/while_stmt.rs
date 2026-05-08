use crate::{Expr, stmt::StmtsBlock};

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub test: Box<Expr>,
    pub body: Box<StmtsBlock>,
}

impl WhileStmt {
    pub fn new(test: Expr, body: StmtsBlock) -> Self {
        Self {
            test: Box::new(test),
            body: Box::new(body),
        }
    }
}
