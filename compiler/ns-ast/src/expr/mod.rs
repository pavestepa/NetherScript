mod binary_expr;
mod binding_expr;
mod call_expr;
mod literal_expr;
mod logical_expr;
mod member_expr;
mod new_expr;
mod referencing;
mod struct_literal_expr;
mod unary_expr;

pub use binary_expr::{BinaryExpr, BinaryOperator};
pub use binding_expr::BindingExpr;
pub use call_expr::CallExpr;
pub use literal_expr::LiteralExpr;
pub use logical_expr::{LogicalExpr, LogicalOperator};
pub use member_expr::{MemberExpr, MemberProperty};
pub use new_expr::NewExpr;
pub use referencing::Referencing;
pub use struct_literal_expr::{StructLiteralExpr, StructLiteralField};
pub use unary_expr::UnaryExpr;

#[derive(Debug, Clone)]
pub enum Expr {
    BindingExpr(BindingExpr),
    CallExpr(CallExpr),
    LiteralExpr(LiteralExpr),
    MemberExpr(MemberExpr),
    BinaryExpr(BinaryExpr),
    LogicalExpr(LogicalExpr),
    Referencing(Referencing),
    UnaryExpr(UnaryExpr),
    NewExpr(NewExpr),
    /// Structural value: a resolved type name plus explicit field initializers.
    StructLiteral(StructLiteralExpr),
}
