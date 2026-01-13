mod block_stmt;
mod break_stmt;
mod expr_stmt;
mod if_stmt;
mod loop_stmt;
mod return_stmt;
mod var_stmt;

pub use block_stmt::BlockStmt;
pub use break_stmt::BreakStmt;
pub use expr_stmt::ExprStmt;
pub use if_stmt::IfStmt;
pub use loop_stmt::LoopStmt;
pub use return_stmt::ReturnStmt;
pub use var_stmt::VarStmt;

#[derive(Debug)]
pub enum Stmt {
    Block(BlockStmt),
    Return(ReturnStmt),
    Break(BreakStmt),
    If(IfStmt),
    Loop(LoopStmt),
    Var(VarStmt),
    Expr(ExprStmt),
}
