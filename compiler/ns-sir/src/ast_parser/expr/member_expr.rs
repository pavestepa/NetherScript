use ns_ast::{MemberExpr, MemberProperty};

use crate::ast_parser::Lowerer;
use crate::expr::SirExpr;

pub(crate) fn lower(lowerer: &mut Lowerer, m: &MemberExpr) -> SirExpr {
    let property = match m.property() {
        MemberProperty::Ident(i) => i.clone().into_simple().as_str().to_string(),
        MemberProperty::Expr(_) => "<computed>".to_string(),
    };
    SirExpr::Member {
        object: Box::new(lowerer.lower_expr(m.object())),
        property,
        ty: None,
    }
}
