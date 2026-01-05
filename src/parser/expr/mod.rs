mod assign_expr;
mod binary_expr;
mod call_expr;
mod class_construct_expr;
mod index_expr;
mod logical_expr;
mod member_expr;
mod unary_expr;

use crate::{ast::Expr, parser::Parser};

impl Parser {
    pub fn parse_expr(&mut self, min_prec: u8) -> Result<Expr, String> {
        // TODO: parse_expr
        Ok(Expr::Boolean(true))
    }
}
