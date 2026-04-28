use ns_ast::ReturnStmt;

use crate::ast_parser::Lowerer;
use crate::stmt::SirStmt;

pub(crate) fn lower(lowerer: &mut Lowerer, r: &ReturnStmt) -> SirStmt {
    SirStmt::Return(r.arg.as_ref().map(|v| lowerer.lower_expr(v)))
}
