use ns_ast::StructLiteralExpr;

use crate::ast_parser::Lowerer;
use crate::expr::SirExpr;

pub(crate) fn lower(lowerer: &mut Lowerer, s: &StructLiteralExpr) -> SirExpr {
    SirExpr::StructLiteral {
        type_name: s.struct_name.clone().into_simple().as_str().to_string(),
        fields: s
            .fields
            .iter()
            .map(|f| {
                (
                    f.ident.clone().into_simple().as_str().to_string(),
                    lowerer.lower_expr(&f.value),
                )
            })
            .collect(),
        ty: None,
    }
}
