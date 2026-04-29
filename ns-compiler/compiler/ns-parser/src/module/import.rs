use ns_ast::Import;
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

impl Parser {
    pub fn parse_import(&mut self) -> Import {
        let ident = self.parse_ident();

        if self.current().kind != TokenKind::Keyword(Keyword::From) {
            self.panic_at_current("expected `from` in import declaration");
        }
        self.parse(TokenKind::Keyword(Keyword::From));

        let mut from = vec![self.parse_ident()];
        while self.current().kind == TokenKind::Dot {
            self.parse(TokenKind::Dot);
            from.push(self.parse_ident());
        }

        self.parse_optional_stmt_delimiter();

        Import { ident, from }
    }
}
