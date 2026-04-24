use ns_ast::{DynamicType, Ident, RefKind, TypeNode, ast::Ast};
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

mod named_type;
mod type_parameter;

impl Parser {
    pub fn parse_type_node(&mut self) -> Ast<TypeNode> {
        match self.current().kind {
            TokenKind::Ampersand => {
                self.parse(TokenKind::Ampersand);
                match self.current().kind {
                    TokenKind::Keyword(Keyword::Mut) => {
                        self.parse(TokenKind::Keyword(Keyword::Mut));
                        match self.current().kind {
                            TokenKind::Keyword(Keyword::Dynamic) => {
                                self.parse_dynamic_type(RefKind::Mut)
                            }
                            _ => Ast::Error(
                                "expected `dynamic` after `&mut` in dynamic interface type".to_string(),
                            ),
                        }
                    }
                    TokenKind::Keyword(Keyword::Dynamic) => self.parse_dynamic_type(RefKind::Ref),
                    _ => Ast::Error(
                        "after `&` in type position, expected `mut dynamic …` or `dynamic …`".to_string(),
                    ),
                }
            }
            TokenKind::Keyword(Keyword::Ref) => {
                self.parse(TokenKind::Keyword(Keyword::Ref));
                match self.current().kind {
                    TokenKind::Keyword(Keyword::Dynamic) => self.parse_dynamic_type(RefKind::Ref),
                    _ => Ast::Error("expected `dynamic` after `ref` in type position".to_string()),
                }
            }
            TokenKind::Keyword(Keyword::Mut) => {
                self.parse(TokenKind::Keyword(Keyword::Mut));
                match self.current().kind {
                    TokenKind::Keyword(Keyword::Dynamic) => self.parse_dynamic_type(RefKind::Mut),
                    _ => Ast::Error("expected `dynamic` after `mut` in type position".to_string()),
                }
            }
            TokenKind::Keyword(Keyword::Dynamic) => self.parse_dynamic_type(RefKind::Own),
            TokenKind::Ident(_) => self.parse_named_type(),
            other => Ast::Error(format!("expected type (name or `dynamic`), found {:?}", other)),
        }
    }

    fn parse_dynamic_type(&mut self, ref_kind: RefKind) -> Ast<TypeNode> {
        self.parse(TokenKind::Keyword(Keyword::Dynamic));
        let TokenKind::Ident(atom) = self.current().kind else {
            return Ast::Error("expected interface name after `dynamic`".to_string());
        };
        let interface = Ident::new(atom);
        self.parse(TokenKind::Ident(atom));
        Ast::Parsed(TypeNode::Dynamic(DynamicType {
            ref_kind,
            interface,
        }))
    }
}
