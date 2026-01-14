use crate::{
    ast::{Expr, ExprStmt},
    parser::Parser,
};

impl Parser {
    pub fn parse_expr_stmt(&mut self) -> Result<ExprStmt, String> {
        println!(
            "~~~~~~~~~~~~~~~~~starting parse_expr_stmt now: {:?}",
            self.peek().unwrap()
        );
        let expr: Expr = self.parse_expr(0)?;
        println!(
            "~~~~~~~~~~~~~~~~~ending parse_expr_stmt now: {:?}",
            self.peek().unwrap()
        );

        Ok(ExprStmt::new(expr))
    }
}
