use ns_ast::Expr;

use crate::ast_parser::Lowerer;
use crate::expr::SirExpr;

mod binary_expr;
mod binding_expr;
mod call_expr;
mod literal_expr;
mod logical_expr;
mod member_expr;
mod new_expr;
mod referencing_expr;
mod struct_literal_expr;
mod template_string_expr;
mod unary_expr;

pub(crate) fn lower_expr(lowerer: &mut Lowerer, expr: &Expr) -> SirExpr {
    match expr {
        Expr::Error(_) => SirExpr::Error { ty: None },
        Expr::LiteralExpr(lit) => literal_expr::lower(lowerer, lit),
        Expr::BindingExpr(b) => binding_expr::lower(lowerer, b),
        Expr::CallExpr(c) => call_expr::lower(lowerer, c),
        Expr::UnaryExpr(u) => unary_expr::lower(lowerer, u),
        Expr::BinaryExpr(b) => binary_expr::lower(lowerer, b),
        Expr::LogicalExpr(l) => logical_expr::lower(lowerer, l),
        Expr::Referencing(r) => referencing_expr::lower(lowerer, r),
        Expr::MemberExpr(m) => lower_member_expr(lowerer, m),
        Expr::StructLiteral(s) => struct_literal_expr::lower(lowerer, s),
        Expr::NewExpr(n) => new_expr::lower(lowerer, n),
        Expr::TemplateString(t) => template_string_expr::lower(lowerer, t),
    }
}

pub(crate) fn lower_member_expr(lowerer: &mut Lowerer, m: &ns_ast::MemberExpr) -> SirExpr {
    member_expr::lower(lowerer, m)
}
