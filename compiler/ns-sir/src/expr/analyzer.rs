use crate::expr::SirExpr;

pub fn contains_error_expr(expr: &SirExpr) -> bool {
    match expr {
        SirExpr::Error { .. } => true,
        SirExpr::Unary { value, .. } => contains_error_expr(value),
        SirExpr::Binary { left, right, .. } => contains_error_expr(left) || contains_error_expr(right),
        SirExpr::Member { object, .. } => contains_error_expr(object),
        SirExpr::StructLiteral { fields, .. } => fields.iter().any(|(_, e)| contains_error_expr(e)),
        SirExpr::New { args, .. } => args.iter().any(contains_error_expr),
        SirExpr::Call { args, .. } => args.iter().any(contains_error_expr),
        SirExpr::IntrinsicPrintln { args, .. } => args.iter().any(contains_error_expr),
        SirExpr::TemplateString { parts, .. } => parts.iter().any(|part| match part {
            crate::expr::SirTemplatePart::Text(_) => false,
            crate::expr::SirTemplatePart::Expr(expr) => contains_error_expr(expr),
        }),
        _ => false,
    }
}
