mod binary_expr;
mod class_construct_expr;
mod fn_call_expr;
mod logical_expr;

use crate::{ast::expr::class_construct_expr::ClassConstructExpr, Atom};
pub use binary_expr::{BinaryExpr, BinaryOperator};
pub use fn_call_expr::FnCallExpr;
pub use logical_expr::{LogicalExpr, LogicalOperator};

#[derive(Debug)]
pub enum Expr {
    This(),
    ArrayLit(),
    Member(),
    Boolean(bool),
    NumberLiteral(Atom),
    StringLiteral(Atom),
    Logical(LogicalExpr),
    Binary(BinaryExpr),
    FnCall(FnCallExpr),
    ClassConstruct(ClassConstructExpr),
}

#[derive(Debug)]
pub struct Property {
    pub key: Atom,
    pub value: Expr,
}
