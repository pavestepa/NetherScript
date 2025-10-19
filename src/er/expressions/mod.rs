pub mod arithmetic_expr;
pub use arithmetic_expr::Arithmetic;
pub mod fn_call_expr;
pub use fn_call_expr::FnCallExpr;
pub mod object_expr;
pub use object_expr::ObjectExpr;
pub mod value_exprs;
pub use value_exprs::ValueExpr;
pub mod var_call_expr;
pub use var_call_expr::VarCallExpr;

pub enum Expr {
    Arithmetic(Arithmetic),
    FnCallExpr(FnCallExpr),
    ObjectExpr(ObjectExpr),
    ValueExpr(ValueExpr),
    VarCallExpr(VarCallExpr),
}
