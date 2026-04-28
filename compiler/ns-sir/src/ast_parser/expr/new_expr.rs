use ns_ast::NewExpr;

use crate::ast_parser::Lowerer;
use crate::expr::SirExpr;

pub(crate) fn lower(lowerer: &mut Lowerer, n: &NewExpr) -> SirExpr {
    SirExpr::New {
        class_name: n.class_ident.clone().into_simple().as_str().to_string(),
        args: n.args.iter().map(|a| lowerer.lower_expr(a)).collect(),
        ty: None,
    }
}
