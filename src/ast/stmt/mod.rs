mod assign_stmt;
mod binding_stmt;
mod break_stmt;
mod expr_stmt;
mod if_stmt;
mod loop_stmt;
mod return_stmt;

pub use assign_stmt::AssignStmt;
pub use binding_stmt::BindingStmt;
pub use break_stmt::BreakStmt;
pub use expr_stmt::ExprStmt;
pub use if_stmt::IfStmt;
pub use loop_stmt::LoopStmt;
pub use return_stmt::ReturnStmt;

use crate::ast::ast::Ast;

#[derive(Debug)]
pub enum Stmt {
    Assign(Ast<AssignStmt>),
    Binding(Ast<BindingStmt>),
    Break(Ast<BreakStmt>),
    Expr(Ast<ExprStmt>),
    If(Ast<IfStmt>),
    Loop(Ast<LoopStmt>),
    Return(Ast<ReturnStmt>),
    Error,
}

#[derive(Debug)]
pub struct StmtsBlock {
    pub stmts: Ast<Vec<Stmt>>,
}
