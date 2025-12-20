mod assign_expr;
mod binary_expr;
mod call_expr;
mod class_construct_expr;
mod fn_call_expr;
mod index_expr;
mod logical_expr;
mod property_expr;
mod ternary_expr;
mod unary_expr;
mod var_call_expr;

use crate::Atom;
pub use assign_expr::{AssignExpr, AssignOperator};
pub use binary_expr::{BinaryExpr, BinaryOperator};
pub use call_expr::CallExpr;
pub use class_construct_expr::ClassConstructExpr;
pub use fn_call_expr::FnCallExpr;
pub use index_expr::IndexExpr;
pub use logical_expr::{LogicalExpr, LogicalOperator};
pub use property_expr::PropertyExpr;
pub use ternary_expr::TernaryExpr;
pub use unary_expr::{UnaryExpr, UnaryOperator};
pub use var_call_expr::VarCallExpr;

#[derive(Debug)]
pub enum Expr {
    Assign(AssignExpr),
    This(),
    ArrayLiteral(),
    Member(),
    Boolean(bool),
    NumberLiteral(Atom),
    StringLiteral(Atom),
    Logical(LogicalExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Call(CallExpr),
    FnCall(FnCallExpr),
    VarCall(VarCallExpr),
    ClassConstruct(ClassConstructExpr),
    Property(PropertyExpr),
    Index(IndexExpr),
    Ternary(TernaryExpr),
}

#[derive(Debug)]
pub struct Property {
    pub key: Atom,
    pub value: Expr,
}
