use ns_ast::{Ident, NamedType, TypeNode, ast::Ast};
use ns_lexer::TokenKind;

use crate::Parser;

impl Parser {
    pub fn parse_named_type(&mut self) -> Ast<TypeNode> {
        let TokenKind::Ident(atom) = self.current().kind else {
            return Ast::Error("expected type name identifier".to_string());
        };
        let ident = Ident::new(atom);
        self.parse(TokenKind::Ident(atom));

        match self.parse_type_arguments() {
            Ok(type_arguments) => Ast::Parsed(TypeNode::Named(NamedType {
                ident,
                type_arguments,
            })),
            Err(e) => Ast::Error(e),
        }
    }

    fn parse_type_arguments(&mut self) -> Result<Vec<TypeNode>, String> {
        if self.current().kind != TokenKind::Less {
            return Ok(Vec::new());
        }
        self.parse(TokenKind::Less);
        let mut args = Vec::new();
        loop {
            if self.current().kind == TokenKind::Greater {
                self.parse(TokenKind::Greater);
                break;
            }
            match self.parse_type_node() {
                Ast::Parsed(node) => args.push(node),
                Ast::Error(e) => return Err(e),
            }
            match self.current().kind {
                TokenKind::Comma => {
                    self.parse(TokenKind::Comma);
                }
                TokenKind::Greater => {}
                other => {
                    return Err(format!(
                        "expected ',' or '>' in type arguments, found {:?}",
                        other
                    ));
                }
            }
        }
        Ok(args)
    }
}
