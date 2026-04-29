use ns_ast::InterfaceDecl;
use ns_lexer::TokenKind;

use crate::Parser;

impl Parser {
    pub fn parse_interface_decl(&mut self) -> InterfaceDecl {
        let ident = self.parse_ident();
        let type_parameters = self.parse_type_parameters_in_angle_brackets();
        self.consume_newlines();

        self.parse(TokenKind::LeftBrace);
        self.consume_newlines();

        let mut methods = Vec::new();
        while self.current().kind != TokenKind::RightBrace {
            if !matches!(self.current().kind, TokenKind::Ident(_)) {
                self.panic_at_current("expected interface method signature");
            }

            let signature = self.parse_callable_signature(true);
            let body = if self.current().kind == TokenKind::LeftBrace {
                Some(self.parse_stmts_block())
            } else {
                None
            };
            methods.push((signature, body));
            self.parse_optional_stmt_delimiter();
        }

        self.parse(TokenKind::RightBrace);

        InterfaceDecl {
            ident,
            type_parameters,
            methods,
        }
    }
}
