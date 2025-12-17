use crate::ast::Stmt;

#[derive(Debug)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}

impl BlockStmt {
    pub fn new() -> Self {
        Self { stmts: vec![] }
    }
    pub fn push(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }
}
