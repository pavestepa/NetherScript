use ns_ast::Referencing;

use crate::ast_parser::Lowerer;
use crate::expr::SirExpr;

pub(crate) fn lower(lowerer: &mut Lowerer, r: &Referencing) -> SirExpr {
    lowerer.lower_expr(&r.expr)
}
