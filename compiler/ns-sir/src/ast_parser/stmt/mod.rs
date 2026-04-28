use ns_ast::Stmt;

use crate::ast_parser::Lowerer;
use crate::stmt::SirStmt;

mod assign_stmt;
mod binding_stmt;
mod break_stmt;
mod expr_stmt;
mod if_stmt;
mod loop_stmt;
mod return_stmt;
mod while_stmt;

pub(crate) fn lower_stmt(lowerer: &mut Lowerer, stmt: &Stmt) -> SirStmt {
    match stmt {
        Stmt::Error(_) => SirStmt::Error,
        Stmt::Expr(s) => expr_stmt::lower(lowerer, s),
        Stmt::Return(r) => return_stmt::lower(lowerer, r),
        Stmt::Binding(b) => binding_stmt::lower(lowerer, b),
        Stmt::Assign(a) => assign_stmt::lower(lowerer, a),
        Stmt::If(i) => if_stmt::lower(lowerer, i),
        Stmt::Loop(l) => loop_stmt::lower(lowerer, l),
        Stmt::While(w) => while_stmt::lower(lowerer, w),
        Stmt::Break(b) => break_stmt::lower(b),
    }
}
