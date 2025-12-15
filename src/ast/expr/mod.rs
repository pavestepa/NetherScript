mod binary_expr;
mod class_construct_expr;
mod call_fn_expr;
mod logical_expr;

use crate::{Atom, ast::expr::{call_fn_expr::CallFnExpr, class_construct_expr::ClassConstructExpr}};
use binary_expr::BinaryExpr;
use logical_expr::LogicalExpr;

#[derive(Debug)]
pub enum Expr {
    NumberLiteral(Atom),
    StringLiteral(Atom),
    Logical(LogicalExpr),
    Binary(BinaryExpr),
    CallFn(CallFnExpr),
    ClassConstruct(ClassConstructExpr)
}

#[derive(Debug)]
pub struct Property {
    pub key: Atom,
    pub value: Expr,
}