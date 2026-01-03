mod assign_expr;
mod binary_expr;
mod call_expr;
mod class_construct_expr;
mod index_expr;
mod logical_expr;
mod unary_expr;

use crate::Atom;
pub use assign_expr::{AssignExpr, AssignOperator};
pub use binary_expr::{BinaryExpr, BinaryOperator};
pub use call_expr::CallExpr;
pub use class_construct_expr::ClassConstructExpr;
pub use index_expr::IndexExpr;
pub use logical_expr::{LogicalExpr, LogicalOperator};
pub use unary_expr::{UnaryExpr, UnaryOperator};

#[derive(Debug)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Boolean(bool),
    Call(CallExpr),
    ClassConstruct(ClassConstructExpr),
    Index(IndexExpr),
    Logical(LogicalExpr),
    NumberLiteral(Atom),
    StringLiteral(Atom),
    Unary(UnaryExpr),
}

#[derive(Debug)]
pub struct Property {
    pub key: Atom,
    pub value: Expr,
}
