use ns_ast::{TemplateStringExpr, TemplateStringPart};

use crate::ast_parser::Lowerer;
use crate::expr::{SirExpr, SirTemplatePart};

pub(crate) fn lower(lowerer: &mut Lowerer, template: &TemplateStringExpr) -> SirExpr {
    let parts = template
        .parts
        .iter()
        .map(|part| match part {
            TemplateStringPart::Text(text) => SirTemplatePart::Text(text.clone()),
            TemplateStringPart::Expr(expr) => {
                SirTemplatePart::Expr(Box::new(lowerer.lower_expr(expr.as_ref())))
            }
        })
        .collect();

    SirExpr::TemplateString {
        parts,
        ty: lowerer.named_type("string"),
    }
}
