use crate::ast::stmt::BlockStmt;

#[derive(Debug)]
pub struct LoopStmt {
    pub body: Box<BlockStmt>,
}

impl LoopStmt {
    pub fn new(body: BlockStmt) -> Self {
        Self {
            body: Box::new(body),
        }
    }
}
