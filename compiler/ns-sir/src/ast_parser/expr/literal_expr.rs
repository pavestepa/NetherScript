use ns_ast::LiteralExpr;

use crate::ast_parser::Lowerer;
use crate::expr::{SirExpr, SirLiteral};

pub(crate) fn lower(lowerer: &mut Lowerer, lit: &LiteralExpr) -> SirExpr {
    match lit {
        LiteralExpr::Number(v) => SirExpr::Literal {
            lit: SirLiteral::Number(v.as_str().to_string()),
            ty: lowerer.named_type("i32"),
        },
        LiteralExpr::String(v) => SirExpr::Literal {
            lit: SirLiteral::String(v.as_str().to_string()),
            ty: lowerer.named_type("string"),
        },
        LiteralExpr::Boolean(v) => SirExpr::Literal {
            lit: SirLiteral::Boolean(v.as_str().as_ref() == "true"),
            ty: lowerer.bool_type(),
        },
    }
}
