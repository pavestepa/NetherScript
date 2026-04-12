use ns_ast::{ExprStmt, ast::Ast};
use ns_lexer::TokenKind;

use crate::Parser;



impl Parser {
    pub fn parse_expr_stmt(&mut self) -> Ast<ExprStmt> {
        let expr = self.parse_expr(0);
        self.parse(TokenKind::Semicolon);
        if let Ast::Error(e) = expr {
            return Ast::Error(e);
        }
        Ast::Parsed(ExprStmt::new(expr))
    }
}
