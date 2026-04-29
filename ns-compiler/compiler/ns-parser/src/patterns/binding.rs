use ns_ast::{Binding, TypedBinding};
use ns_lexer::TokenKind;

use crate::Parser;

impl Parser {
    pub fn parse_binding(&mut self) -> Binding {
        let ident = self.parse_ident();
        let type_ref = if self.current().kind == TokenKind::Colon {
            self.parse(TokenKind::Colon);
            Some(self.parse_type_node())
        } else {
            None
        };
        Binding { ident, type_ref }
    }

    pub fn parse_typed_binding(&mut self) -> TypedBinding {
        let ident = self.parse_ident();
        self.parse(TokenKind::Colon);
        let type_ref = self.parse_type_node();
        TypedBinding { ident, type_ref }
    }
}
