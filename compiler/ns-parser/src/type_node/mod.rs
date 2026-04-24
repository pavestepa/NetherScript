use ns_ast::{DynamicType, Ident, TypeNode, ast::Ast};
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

mod named_type;
mod type_parameter;

impl Parser {
    pub fn parse_type_node(&mut self) -> Ast<TypeNode> {
        match self.current().kind {
            TokenKind::Keyword(Keyword::Dynamic) => self.parse_dynamic_type(),
            TokenKind::Ident(_) => self.parse_named_type(),
            other => Ast::Error(format!("expected type (name or `dynamic`), found {:?}", other)),
        }
    }

    fn parse_dynamic_type(&mut self) -> Ast<TypeNode> {
        self.parse(TokenKind::Keyword(Keyword::Dynamic));
        let TokenKind::Ident(atom) = self.current().kind else {
            return Ast::Error("expected interface name after `dynamic`".to_string());
        };
        let interface = Ident::new(atom);
        self.parse(TokenKind::Ident(atom));
        Ast::Parsed(TypeNode::Dynamic(DynamicType { interface }))
    }
}
