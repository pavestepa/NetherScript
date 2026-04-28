use ns_ast::LoopStmt;

use crate::ast_parser::Lowerer;
use crate::stmt::SirStmt;

pub(crate) fn lower(lowerer: &mut Lowerer, l: &LoopStmt) -> SirStmt {
    lowerer.push_scope();
    let body = l.body.stmts.iter().map(|s| lowerer.lower_stmt(s)).collect();
    lowerer.pop_scope();
    SirStmt::Loop { body }
}
