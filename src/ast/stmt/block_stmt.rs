use crate::ast::{ast::Ast, Stmt};

#[derive(Debug)]
pub struct BlockStmt {
    pub stmts: Ast<Vec<Stmt>>,
}
