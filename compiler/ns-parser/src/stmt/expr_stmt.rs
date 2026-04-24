use ns_ast::ExprStmt;

use crate::Parser;

impl Parser {
    pub fn parse_expr_stmt(&mut self) -> ExprStmt {
        let expr = self.parse_expr(0);
        self.parse_optional_stmt_delimiter();
        ExprStmt::new(expr)
    }
}
