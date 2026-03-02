use crate::{
    ast::{ast::Ast, Expr},
    parser::Parser,
};

impl Parser {
    pub fn parse_function_call_expr() -> Ast<Expr> {
        Ast::Error("".to_string())
    }
}
