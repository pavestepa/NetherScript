mod assign_stmt;
mod binding_stmt;
mod break_stmt;
mod call_stmt;
mod expr_stmt;
mod if_stmt;
mod loop_stmt;
mod return_stmt;
mod while_stmt;

pub use assign_stmt::{AssignStmt, AssignTarget};
pub use binding_stmt::BindingStmt;
pub use break_stmt::BreakStmt;
pub use call_stmt::CallStmt;
pub use expr_stmt::ExprStmt;
pub use if_stmt::IfStmt;
pub use loop_stmt::LoopStmt;
pub use return_stmt::ReturnStmt;
pub use while_stmt::WhileStmt;

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign(AssignStmt),
    Binding(BindingStmt),
    Break(BreakStmt),
    Expr(ExprStmt),
    If(IfStmt),
    Loop(LoopStmt),
    Return(ReturnStmt),
    While(WhileStmt),
}

#[derive(Debug, Clone)]
pub struct StmtsBlock {
    pub stmts: Vec<Stmt>,
}
