use ns_ast::IfStmt;
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

impl Parser {
    pub fn parse_if_stmt(&mut self) -> IfStmt {
        let test = self.parse_expr(0);
        let body = self.parse_stmts_block();
        let alt = if self.current().kind == TokenKind::Keyword(Keyword::Else) {
            self.parse(TokenKind::Keyword(Keyword::Else));
            Some(self.parse_stmts_block())
        } else {
            None
        };
        IfStmt::new(test, body, alt)
    }
}
