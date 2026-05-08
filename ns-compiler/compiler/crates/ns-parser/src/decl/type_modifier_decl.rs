use ns_ast::{ExtendsDecl, ImplementsDecl, StmtsBlock, TypeModifierDecl};
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

impl Parser {
    pub fn parse_type_modifier_decl(&mut self) -> TypeModifierDecl {
        let type_identifier = self.parse_type_node();
        self.consume_newlines();

        match self.current().kind {
            TokenKind::Keyword(Keyword::Extends) => {
                self.parse(TokenKind::Keyword(Keyword::Extends));
                self.consume_newlines();
                let methods = self.parse_type_modifier_methods();
                TypeModifierDecl::Extends(ExtendsDecl {
                    type_identifier,
                    methods,
                })
            }
            TokenKind::Keyword(Keyword::Implements) => {
                self.parse(TokenKind::Keyword(Keyword::Implements));
                let mut interfaces = vec![self.parse_type_node()];
                while self.current().kind == TokenKind::Comma {
                    self.parse(TokenKind::Comma);
                    interfaces.push(self.parse_type_node());
                }
                self.consume_newlines();

                let methods = self.parse_type_modifier_methods();
                TypeModifierDecl::Implements(ImplementsDecl {
                    type_identifier,
                    interfaces,
                    methods,
                })
            }
            other => self.panic_at_current(format!(
                "expected `extends` or `implements` after type name, found {:?}",
                other
            )),
        }
    }

    fn parse_type_modifier_methods(&mut self) -> Vec<(ns_ast::Callable, StmtsBlock)> {
        self.parse(TokenKind::LeftBrace);
        self.consume_newlines();

        let mut methods = Vec::new();
        while self.current().kind != TokenKind::RightBrace {
            if !matches!(self.current().kind, TokenKind::Ident(_)) {
                self.panic_at_current("expected method declaration in type modifier block");
            }
            let signature = self.parse_callable_signature(true);
            let body = self.parse_stmts_block();
            methods.push((signature, body));
            self.parse_optional_stmt_delimiter();
        }

        self.parse(TokenKind::RightBrace);
        methods
    }
}
