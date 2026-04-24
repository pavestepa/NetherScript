mod binary_expr;
mod binding_expr;
mod call_expr;
mod literal_expr;
mod logical_expr;
mod member_expr;
mod referencing;
mod struct_literal_expr;
mod unary_expr;

pub use binary_expr::{BinaryExpr, BinaryOperator};
pub use binding_expr::BindingExpr;
pub use call_expr::CallExpr;
pub use literal_expr::LiteralExpr;
pub use logical_expr::{LogicalExpr, LogicalOperator};
pub use member_expr::{MemberExpr, MemberProperty};
pub use referencing::Referencing;
pub use struct_literal_expr::{StructLiteralExpr, StructLiteralField};
pub use unary_expr::UnaryExpr;

#[derive(Debug, Clone)]
pub enum Expr {
    //                             example             second token
    BindingExpr(BindingExpr),   // a
    CallExpr(CallExpr), // a()                 (
    LiteralExpr(LiteralExpr),   // true, "text", 4
    MemberExpr(MemberExpr),     // e.a                 .
    BinaryExpr(BinaryExpr),         // e + e               +
    LogicalExpr(LogicalExpr),       // e > e               >
    Referencing(Referencing),   // read e              e
    UnaryExpr(UnaryExpr),
    /// `User { id: id, name: "Paul" }` — Rust-style struct literal.
    StructLiteral(StructLiteralExpr),
}
