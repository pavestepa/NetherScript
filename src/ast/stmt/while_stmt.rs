use crate::ast::Stmt;

#[derive(Debug)]
pub struct WhileStmt {
    pub test: Box<Stmt>,
    pub body: Box<Stmt>,
}
