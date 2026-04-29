use ns_ast::Decl;
use ns_lexer::{Keyword, TokenKind};

use super::parser::Parser;

mod class_decl;
mod const_decl;
mod enum_decl;
mod function_decl;
mod interface_decl;
mod type_decl;
mod type_modifier_decl;

impl Parser {
    pub fn parse_decl(&mut self) -> Decl {
        self.consume_newlines();
        if let TokenKind::Keyword(keyword) = self.current().kind {
            match keyword {
                Keyword::Class => {
                    self.parse(TokenKind::Keyword(keyword));
                    Decl::Class(self.parse_class_decl())
                }
                Keyword::Function => {
                    self.parse(TokenKind::Keyword(keyword));
                    Decl::Function(self.parse_function_decl())
                }
                Keyword::Enum => {
                    self.parse(TokenKind::Keyword(keyword));
                    Decl::Enum(self.parse_enum_decl())
                }
                Keyword::Interface => {
                    self.parse(TokenKind::Keyword(keyword));
                    Decl::Interface(self.parse_interface_decl())
                }
                Keyword::Const => {
                    self.parse(TokenKind::Keyword(keyword));
                    Decl::Const(self.parse_const_decl())
                }
                Keyword::Type => {
                    self.parse(TokenKind::Keyword(keyword));
                    Decl::Type(self.parse_type_decl())
                }
                other => self.panic_at_current(format!("unsupported declaration keyword {:?}", other)),
            }
        } else if self.looks_like_type_modifier_decl_start() {
            Decl::TypeModifier(self.parse_type_modifier_decl())
        } else {
            self.panic_at_current(format!(
                "Token {:?} is not keyword and not suitable for Decl",
                self.current().kind
            ))
        }
    }
}
