mod binary_op;
mod binding_call;
mod function_call;
mod literal_call;
mod logical_op;
mod member_call;
mod unary_op;

use crate::{
    ast::{ast::Ast, Expr},
    parser::Parser,
};

impl Parser {
    pub fn parse_expr(&mut self) -> Ast<Expr> {
        /* implement expr parsing */
        Ast::Error("".to_string())
    }
}
