use ns_ast::BinaryExpr;

use crate::ast_parser::Lowerer;
use crate::expr::SirExpr;

pub(crate) fn lower(lowerer: &mut Lowerer, b: &BinaryExpr) -> SirExpr {
    SirExpr::Binary {
        op: format!("{:?}", b.kind),
        left: Box::new(lowerer.lower_expr(&b.left)),
        right: Box::new(lowerer.lower_expr(&b.right)),
        ty: None,
    }
}
