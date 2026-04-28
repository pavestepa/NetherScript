use ns_ast::WhileStmt;

use crate::ast_parser::Lowerer;
use crate::expr::SirExpr;
use crate::stmt::SirStmt;

pub(crate) fn lower(lowerer: &mut Lowerer, w: &WhileStmt) -> SirStmt {
    lowerer.push_scope();
    let mut body = Vec::with_capacity(w.body.stmts.len() + 1);
    let test = lowerer.lower_expr(&w.test);
    body.push(SirStmt::If {
        test: SirExpr::Unary {
            op: "!".to_string(),
            value: Box::new(test),
            ty: lowerer.bool_type(),
        },
        then_body: vec![SirStmt::Break],
        else_body: Vec::new(),
    });
    body.extend(w.body.stmts.iter().map(|s| lowerer.lower_stmt(s)));
    lowerer.pop_scope();
    SirStmt::Loop { body }
}
