use ns_ast::BreakStmt;

use crate::stmt::SirStmt;

pub(crate) fn lower(_b: &BreakStmt) -> SirStmt {
    SirStmt::Break
}
