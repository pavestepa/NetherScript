use crate::semantics::hir::stmts::{
    assign_stmt::AssignStmt, binding_stmt::BindingStmt, break_stmt::BreakStmt, expr_stmt::ExprStmt,
    if_stmt::IfStmt, loop_stmt::LoopStmt, return_stmt::ReturnStmt,
};

mod assign_stmt;
mod binding_stmt;
mod break_stmt;
mod call_stmt;
mod expr_stmt;
mod if_stmt;
mod loop_stmt;
mod return_stmt;

pub struct Stmts(Vec<Stmt>);

pub enum Stmt {
    Assign(AssignStmt),
    Binding(BindingStmt),
    Break(BreakStmt),
    Expr(ExprStmt),
    If(IfStmt),
    Loop(LoopStmt),
    Return(ReturnStmt),
}
