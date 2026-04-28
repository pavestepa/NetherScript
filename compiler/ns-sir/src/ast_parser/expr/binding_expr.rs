use ns_ast::BindingExpr;

use crate::ast_parser::Lowerer;
use crate::expr::SirExpr;

pub(crate) fn lower(lowerer: &mut Lowerer, b: &BindingExpr) -> SirExpr {
    let name = b.0.clone().into_simple().as_str().to_string();
    lowerer
        .lookup_value(&name)
        .map(|value| SirExpr::ValueRef {
            value,
            ty: lowerer.value_type(value),
        })
        .unwrap_or(SirExpr::Error { ty: None })
}
