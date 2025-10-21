use crate::ast::expressions::Expr;

pub struct VarDecl {
    is_const: bool,
    name: String,
    assign: Expr,
}
