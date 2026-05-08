use ns_ast::Ident;
use ns_lexer::TokenKind;

use crate::Parser;

impl Parser {
    pub fn parse_ident(&mut self) -> Ident {
        match self.current().kind {
            TokenKind::Ident(ident) => {
                self.parse(TokenKind::Ident(ident));
                Ident::new(ident)
            }
            other => self.panic_at_current(format!("expected identifier, found {:?}", other)),
        }
    }
}
