use ns_ast::{NamedType, TypeNode};
use ns_lexer::TokenKind;

use crate::Parser;

impl Parser {
    pub fn parse_named_type(&mut self) -> TypeNode {
        let ident = self.parse_ident();
        let type_arguments = self.parse_type_arguments();
        TypeNode::Named(NamedType::new(ident, type_arguments))
    }

    pub fn parse_type_arguments(&mut self) -> Vec<TypeNode> {
        if self.current().kind != TokenKind::Less {
            return Vec::new();
        }

        self.parse(TokenKind::Less);

        let mut args = Vec::new();
        loop {
            if self.current().kind == TokenKind::Greater {
                self.parse(TokenKind::Greater);
                break;
            }

            args.push(self.parse_type_node());

            match self.current().kind {
                TokenKind::Comma => {
                    self.parse(TokenKind::Comma);
                }
                TokenKind::Greater => {}
                other => {
                    self.panic_at_current(format!(
                        "expected ',' or '>' in type arguments, found {:?}",
                        other
                    ));
                }
            }
        }

        args
    }
}
