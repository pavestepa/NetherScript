use crate::{
    ast::{stmt::ExprStmt, Expr},
    parser::Parser,
};

impl Parser {
    pub fn parse_expr_stmt(&mut self) -> Result<ExprStmt, String> {
        // TODO: parse_expr_stmt
        Ok(ExprStmt::new(Expr::Boolean(true)))
    }
}
