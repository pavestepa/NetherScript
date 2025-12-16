mod binary_expr;
mod class_construct_expr;
mod fn_call_expr;
mod logical_expr;

use crate::{Atom, ast::expr::class_construct_expr::ClassConstructExpr};
pub use binary_expr::{ BinaryExpr, BinaryOperator };
pub use logical_expr::{ LogicalExpr, LogicalOperator };
pub use fn_call_expr::FnCallExpr;

#[derive(Debug)]
pub enum Expr {
    Boolean(bool),
    NumberLiteral(Atom),
    StringLiteral(Atom),
    Logical(LogicalExpr),
    Binary(BinaryExpr),
    FnCall(FnCallExpr),
    ClassConstruct(ClassConstructExpr)
}

#[derive(Debug)]
pub struct Property {
    pub key: Atom,
    pub value: Expr,
}