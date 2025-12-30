mod precedence;
use std::fmt::format;

use crate::{
    ast::{
        expr::{
            AssignExpr, AssignOperator, BinaryExpr, BinaryOperator, CallExpr, IndexExpr,
            PropertyExpr, TernaryExpr, UnaryExpr, UnaryOperator,
        },
        Expr,
    },
    lexer::Token,
    parser::Parser,
};
use precedence::precedence;

impl Parser {
    pub fn parse_expr(&mut self, min_prec: u8) -> Result<Expr, String> {
        // TODO: parse_expr
        Ok(Expr::Boolean(true))
    }
}
