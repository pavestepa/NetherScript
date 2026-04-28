use ns_ast::{AssignStmt, AssignTarget};

use crate::ast_parser::Lowerer;
use crate::expr::SirExpr;
use crate::stmt::SirStmt;

pub(crate) fn lower(lowerer: &mut Lowerer, a: &AssignStmt) -> SirStmt {
    let target = match &a.target {
        AssignTarget::Ident(i) => {
            let name = i.clone().into_simple().as_str().to_string();
            lowerer
                .lookup_value(&name)
                .map(|value| SirExpr::ValueRef {
                    value,
                    ty: lowerer.value_type(value),
                })
                .unwrap_or(SirExpr::Error { ty: None })
        }
        AssignTarget::Member(m) => crate::ast_parser::expr::lower_member_expr(lowerer, m),
    };
    SirStmt::Assign {
        target,
        value: lowerer.lower_expr(&a.assign),
    }
}
