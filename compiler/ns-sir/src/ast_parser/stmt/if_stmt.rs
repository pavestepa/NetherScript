use ns_ast::IfStmt;

use crate::ast_parser::Lowerer;
use crate::stmt::SirStmt;

pub(crate) fn lower(lowerer: &mut Lowerer, i: &IfStmt) -> SirStmt {
    lowerer.push_scope();
    let then_body = i.body.stmts.iter().map(|s| lowerer.lower_stmt(s)).collect();
    lowerer.pop_scope();

    lowerer.push_scope();
    let else_body = i
        .alt
        .as_ref()
        .map(|b| b.stmts.iter().map(|s| lowerer.lower_stmt(s)).collect())
        .unwrap_or_else(Vec::new);
    lowerer.pop_scope();

    SirStmt::If {
        test: lowerer.lower_expr(&i.test),
        then_body,
        else_body,
    }
}
