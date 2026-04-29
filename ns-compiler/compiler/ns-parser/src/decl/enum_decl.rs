use ns_ast::{EnumDecl, EnumMember};
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

impl Parser {
    pub fn parse_enum_decl(&mut self) -> EnumDecl {
        let ident = self.parse_ident();
        let type_parameters = self.parse_type_parameters_in_angle_brackets();
        self.consume_newlines();

        if self.current().kind == TokenKind::Keyword(Keyword::Extends) {
            self.panic_at_current("enum can not use `extends`; use `implements` only")
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

        let mut members = Vec::new();
        let mut methods = Vec::new();

        while self.current().kind != TokenKind::RightBrace {
            if !matches!(self.current().kind, TokenKind::Ident(_)) {
                self.panic_at_current("expected enum member or method name");
            }

            if self.is_enum_method_start() {
                methods.push(self.parse_method());
                self.parse_optional_stmt_delimiter();
            } else {
                members.push(self.parse_enum_member());
            }
            self.consume_newlines();
        }

        self.parse(TokenKind::RightBrace);

        EnumDecl {
            ident,
            type_parameters,
            implements,
            members,
            methods,
        }
    }

    fn is_enum_method_start(&self) -> bool {
        let mut offset = 1;
        while self.peek(offset).kind == TokenKind::Less {
            let mut depth = 0usize;
            loop {
                let token = self.peek(offset).kind;
                match token {
                    TokenKind::Less => depth += 1,
                    TokenKind::Greater => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            offset += 1;
                            break;
                        }
                    }
                    _ => {}
                }
                offset += 1;
            }
        }

        if self.peek(offset).kind != TokenKind::LeftParen {
            return false;
        }

        let mut paren_depth = 0usize;
        loop {
            let token = self.peek(offset).kind;
            match token {
                TokenKind::LeftParen => paren_depth += 1,
                TokenKind::RightParen => {
                    paren_depth = paren_depth.saturating_sub(1);
                    if paren_depth == 0 {
                        return self.peek(offset + 1).kind == TokenKind::Colon;
                    }
                }
                _ => {}
            }
            offset += 1;
        }
    }

    fn parse_enum_member(&mut self) -> EnumMember {
        let ident = self.parse_ident();

        let contains = if self.current().kind == TokenKind::LeftParen {
            self.parse(TokenKind::LeftParen);
            let ty = self.parse_type_node();
            self.parse(TokenKind::RightParen);
            Some(ty)
        } else {
            None
        };

        if self.current().kind != TokenKind::Comma {
            self.panic_at_current("enum member must end with `,`");
        }
        self.parse(TokenKind::Comma);

        EnumMember { ident, contains }
    }
}
