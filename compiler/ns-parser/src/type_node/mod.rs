use ns_ast::{DynamicType, TypeNode};
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

mod named_type;
pub mod type_parameter;

impl Parser {
    pub fn parse_type_node(&mut self) -> TypeNode {
        match self.current().kind {
            TokenKind::Ampersand => {
                self.parse(TokenKind::Ampersand);
                match self.current().kind {
                    TokenKind::Keyword(Keyword::Mut) => {
                        self.parse(TokenKind::Keyword(Keyword::Mut));
                        match self.current().kind {
                            TokenKind::Keyword(Keyword::Dynamic) => self.parse_dynamic_type(ns_ast::RefKind::Mut),
                            _ => self.panic_at_current(
                                "expected `dynamic` after `&mut` in dynamic interface type",
                            ),
                        }
                    }
                    TokenKind::Keyword(Keyword::Dynamic) => self.parse_dynamic_type(ns_ast::RefKind::Ref),
                    _ => self.panic_at_current(
                        "after `&` in type position, expected `mut dynamic …` or `dynamic …`",
                    ),
                }
            }
            TokenKind::Keyword(Keyword::Ref) => {
                self.parse(TokenKind::Keyword(Keyword::Ref));
                match self.current().kind {
                    TokenKind::Keyword(Keyword::Dynamic) => self.parse_dynamic_type(ns_ast::RefKind::Ref),
                    _ => self.panic_at_current("expected `dynamic` after `ref` in type position"),
                }
            }
            TokenKind::Keyword(Keyword::Mut) => {
                self.parse(TokenKind::Keyword(Keyword::Mut));
                match self.current().kind {
                    TokenKind::Keyword(Keyword::Dynamic) => self.parse_dynamic_type(ns_ast::RefKind::Mut),
                    _ => self.panic_at_current("expected `dynamic` after `mut` in type position"),
                }
            }
            TokenKind::Keyword(Keyword::Dynamic) => self.parse_dynamic_type(ns_ast::RefKind::Own),
            TokenKind::Ident(_) => self.parse_named_type(),
            other => self.panic_at_current(format!("expected type (name or `dynamic`), found {:?}", other)),
        }
    }

    fn parse_dynamic_type(&mut self, ref_kind: ns_ast::RefKind) -> TypeNode {
        self.parse(TokenKind::Keyword(Keyword::Dynamic));
        let TokenKind::Ident(atom) = self.current().kind else {
            self.panic_at_current("expected interface name after `dynamic`");
        };
        let interface = ns_ast::Ident::new(atom);
        self.parse(TokenKind::Ident(atom));
        TypeNode::Dynamic(DynamicType {
            ref_kind,
            interface,
        })
    }
}
