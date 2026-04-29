use ns_ast::BreakStmt;
use ns_lexer::TokenKind;

use crate::Parser;

impl Parser {
    pub fn parse_break_stmt(&mut self) -> BreakStmt {
        let label = if let TokenKind::Ident(_) = self.current().kind {
            Some(self.parse_ident())
        } else {
            None
        };
        self.parse_optional_stmt_delimiter();
        BreakStmt::new(label)
    }
}
