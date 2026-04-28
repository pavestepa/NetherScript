use ns_ast::ExprStmt;

use crate::ast_parser::Lowerer;
use crate::stmt::SirStmt;

pub(crate) fn lower(lowerer: &mut Lowerer, s: &ExprStmt) -> SirStmt {
    SirStmt::Expr(lowerer.lower_expr(&s.expr))
}
