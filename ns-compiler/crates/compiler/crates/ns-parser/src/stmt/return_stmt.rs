use ns_ast::ReturnStmt;
use ns_lexer::TokenKind;

use crate::Parser;

impl Parser {
    pub fn parse_return_stmt(&mut self) -> ReturnStmt {
        let arg = match self.current().kind {
            TokenKind::Semicolon | TokenKind::Newline | TokenKind::RightBrace => None,
            _ => Some(self.parse_expr(0)),
        };
        self.parse_optional_stmt_delimiter();
        ReturnStmt::new(arg)
    }
}
