use ns_ast::UnaryExpr;

use crate::ast_parser::Lowerer;
use crate::expr::SirExpr;

pub(crate) fn lower(lowerer: &mut Lowerer, u: &UnaryExpr) -> SirExpr {
    let (op, value) = match u {
        UnaryExpr::Minus(v) => ("-".to_string(), v),
        UnaryExpr::Plus(v) => ("+".to_string(), v),
        UnaryExpr::Not(v) => ("!".to_string(), v),
        UnaryExpr::BitNot(v) => ("~".to_string(), v),
        UnaryExpr::Deref(v) => ("*".to_string(), v),
    };
    SirExpr::Unary {
        op,
        value: Box::new(lowerer.lower_expr(value)),
        ty: None,
    }
}
