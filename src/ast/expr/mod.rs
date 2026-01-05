mod assign_expr;
mod binary_expr;
mod call_expr;
mod class_construct_expr;
mod index_expr;
mod logical_expr;
mod member_expr;
mod unary_expr;

use crate::Atom;
pub use assign_expr::{AssignExpr, AssignOperator};
pub use binary_expr::{BinaryExpr, BinaryOperator};
pub use call_expr::CallExpr;
pub use class_construct_expr::ClassConstructExpr;
pub use index_expr::IndexExpr;
pub use logical_expr::{LogicalExpr, LogicalOperator};
pub use member_expr::MemberExpr;
pub use unary_expr::UnaryExpr;

#[derive(Debug)]
pub enum Expr {
    NumberLiteral(Atom),
    StringLiteral(Atom),
    Ident(Atom),
    Call(CallExpr),
    Boolean(bool),
    Member(MemberExpr),
    Assign(AssignExpr),
    Binary(BinaryExpr),
    ClassConstruct(ClassConstructExpr),
    Index(IndexExpr),
    Logical(LogicalExpr),
    Unary(UnaryExpr),
}

#[derive(Debug)]
pub struct Property {
    pub key: Atom,
    pub value: Expr,
}
