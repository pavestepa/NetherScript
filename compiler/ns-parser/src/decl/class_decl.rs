use ns_ast::{ClassDecl, Field};
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

impl Parser {
    pub fn parse_class_decl(&mut self) -> ClassDecl {
        let ident = self.parse_ident();
        let type_parameters = self.parse_type_parameters_in_angle_brackets();
        self.consume_newlines();

        let mut extends = None;
        if self.current().kind == TokenKind::Keyword(Keyword::Extends) {
            self.parse(TokenKind::Keyword(Keyword::Extends));
            extends = Some(self.parse_ident());
            self.consume_newlines();
        }

        let mut implements = None;
        if self.current().kind == TokenKind::Keyword(Keyword::Implements) {
            self.parse(TokenKind::Keyword(Keyword::Implements));
            let mut traits = vec![self.parse_ident()];
            while self.current().kind == TokenKind::Comma {
                self.parse(TokenKind::Comma);
                traits.push(self.parse_ident());
            }
            implements = Some(traits);
            self.consume_newlines();
        }

        self.parse(TokenKind::LeftBrace);
        self.consume_newlines();

        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while self.current().kind != TokenKind::RightBrace {
            match (self.current().kind, self.peek(1).kind) {
                (TokenKind::Ident(_), TokenKind::Colon) => {
                    fields.push(self.parse_field());
                    self.parse_optional_stmt_delimiter();
                }
                (TokenKind::Ident(_), TokenKind::LeftParen)
                | (TokenKind::Ident(_), TokenKind::Less) => {
                    methods.push(self.parse_method());
                    self.parse_optional_stmt_delimiter();
                }
                (cur, next) => {
                    self.panic_at_current(format!(
                        "expected class field or method, got current={:?}, next={:?}",
                        cur, next
                    ));
                }
            }
            self.consume_newlines();
        }

        self.parse(TokenKind::RightBrace);

        ClassDecl {
            ident,
            type_parameters,
            extends,
            implements,
            fields,
            methods,
        }
    }

    fn parse_field(&mut self) -> Field {
        let ident = self.parse_ident();
        self.parse(TokenKind::Colon);
        let type_ref = self.parse_type_node();

        let init = if self.peek_non_newline_kind(0) == TokenKind::Assign {
            self.parse_required_after_linebreaks(TokenKind::Assign, "in class field initializer");
            Some(self.parse_expr(0))
        } else {
            None
        };

        Field {
            binding: ns_ast::Binding {
                ident,
                type_ref: Some(type_ref),
            },
            init,
        }
    }
}
