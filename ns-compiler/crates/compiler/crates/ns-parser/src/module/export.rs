use ns_ast::{Decl, Export};
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

impl Parser {
    pub fn parse_export(&mut self) -> Export {
        match self.current().kind {
            TokenKind::LeftBrace => {
                self.parse(TokenKind::LeftBrace);
                let mut idents = Vec::new();
                if self.current().kind != TokenKind::RightBrace {
                    loop {
                        idents.push(self.parse_ident());
                        if self.current().kind == TokenKind::Comma {
                            self.parse(TokenKind::Comma);
                        } else {
                            break;
                        }
                    }
                }
                self.parse(TokenKind::RightBrace);
                self.parse_optional_stmt_delimiter();
                Export::Idents(idents)
            }
            TokenKind::Ident(_) => {
                if matches!(
                    self.peek(1).kind,
                    TokenKind::Keyword(Keyword::Extends | Keyword::Implements)
                ) {
                    self.panic_at_current("type modifier declaration can not be exported");
                }
                let ident = self.parse_ident();
                self.parse_optional_stmt_delimiter();
                Export::Ident(ident)
            }
            other => self.panic_at_current(format!(
                "invalid export syntax, expected ident or '{{...}}', found {:?}",
                other
            )),
        }
    }

    pub fn parse_export_decl_form(&mut self) -> Decl {
        match self.current().kind {
            TokenKind::Keyword(
                Keyword::Class
                | Keyword::Function
                | Keyword::Enum
                | Keyword::Interface
                | Keyword::Const
                | Keyword::Type,
            ) => self.parse_decl(),
            other => self.panic_at_current(format!(
                "expected declaration after `export`, found {:?}",
                other
            )),
        }
    }
}
