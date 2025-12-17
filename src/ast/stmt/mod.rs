mod block_stmt;
mod break_stmt;
mod expr_stmt;
mod if_stmt;
mod return_stmt;
mod var_stmt;
mod while_stmt;

pub use block_stmt::BlockStmt;
pub use break_stmt::BreakStmt;
pub use expr_stmt::ExprStmt;
pub use if_stmt::IfStmt;
pub use return_stmt::ReturnStmt;
pub use var_stmt::VarStmt;
pub use while_stmt::WhileStmt;

#[derive(Debug)]
pub enum Stmt {
    Block(BlockStmt),
    Return(ReturnStmt),
    Break(BreakStmt),
    If(IfStmt),
    While(WhileStmt),
    Var(VarStmt),
    Expr(ExprStmt),
}
