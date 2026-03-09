use crate::syntax::{
    ast::{ast::Ast, RefKind, ReferenceType, TypeNode},
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn parse_reference_type_change(&mut self) -> Ast<TypeNode> {
        self.parse(TokenKind::Keyword(Keyword::Change));

        return Ast::Parsed(TypeNode::ReferenceType(ReferenceType {
            kind: RefKind::Change,
            argument: Box::new(self.parse_type_node()),
        }));
    }
    pub fn parse_reference_type_read(&mut self) -> Ast<TypeNode> {
        self.parse(TokenKind::Keyword(Keyword::Read));

        return Ast::Parsed(TypeNode::ReferenceType(ReferenceType {
            kind: RefKind::Read,
            argument: Box::new(self.parse_type_node()),
        }));
    }
}
