use ns_ast::StmtsBlock;
use ns_lexer::TokenKind;

use crate::Parser;

impl Parser {
    pub fn parse_stmts_block(&mut self) -> StmtsBlock {
        self.parse(TokenKind::LeftBrace);
        self.consume_newlines();

        let mut stmts = Vec::new();
        while self.current().kind != TokenKind::RightBrace {
            stmts.push(self.parse_stmt_recover());
            self.consume_newlines();
        }

        self.parse(TokenKind::RightBrace);
        StmtsBlock { stmts }
    }
}
