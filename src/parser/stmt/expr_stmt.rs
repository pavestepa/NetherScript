use crate::{
    ast::{stmt::ExprStmt, Expr},
    lexer::Token,
    parser::Parser,
};

impl Parser {
    pub fn parse_expr_stmt(&mut self) -> Result<ExprStmt, String> {
        let expr: Expr = self.parse_expr(0)?;

        Ok(ExprStmt::new(expr))
    }
}
