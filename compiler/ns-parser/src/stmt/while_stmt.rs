use ns_ast::WhileStmt;
use ns_lexer::TokenKind;

use crate::Parser;

impl Parser {
    pub fn parse_while_stmt(&mut self) -> WhileStmt {
        self.parse(TokenKind::LeftParen);
        let test = self.parse_expr(0);
        self.parse(TokenKind::RightParen);
        let body = self.parse_stmts_block();
        WhileStmt::new(test, body)
    }
}
