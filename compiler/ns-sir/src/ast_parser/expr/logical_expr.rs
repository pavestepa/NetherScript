use ns_ast::LogicalExpr;

use crate::ast_parser::Lowerer;
use crate::expr::SirExpr;

pub(crate) fn lower(lowerer: &mut Lowerer, l: &LogicalExpr) -> SirExpr {
    SirExpr::Binary {
        op: format!("{:?}", l.kind),
        left: Box::new(lowerer.lower_expr(&l.left)),
        right: Box::new(lowerer.lower_expr(&l.right)),
        ty: lowerer.bool_type(),
    }
}
