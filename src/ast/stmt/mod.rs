pub mod var_stmt;

use crate::{ast::Expr};
use var_stmt::VarStmt;

#[derive(Debug)]
pub enum Stmt {
    VarStmt (VarStmt),
    ExprStmt(Expr), // например: foo()
}
