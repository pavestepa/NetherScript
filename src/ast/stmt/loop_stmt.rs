use crate::ast::stmt::StmtsBlock;

#[derive(Debug)]
pub struct LoopStmt {
    pub body: Box<StmtsBlock>,
}

impl LoopStmt {
    pub fn new(body: StmtsBlock) -> Self {
        Self {
            body: Box::new(body),
        }
    }
}
