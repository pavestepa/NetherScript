use crate::syntax::{
    ast::{ast::Ast, RefKind, ReferenceType, TypeNode},
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn parse_reference_type_own(&mut self) -> Ast<TypeNode> {
        self.parse(TokenKind::Keyword(Keyword::Own));

        return Ast::Parsed(TypeNode::ReferenceType(ReferenceType {
            kind: RefKind::Own,
            argument: Box::new(self.parse_type_node()),
        }));
    }
    pub fn parse_reference_type_ref(&mut self) -> Ast<TypeNode> {
        self.parse(TokenKind::Keyword(Keyword::Ref));

        return Ast::Parsed(TypeNode::ReferenceType(ReferenceType {
            kind: RefKind::Ref,
            argument: Box::new(self.parse_type_node()),
        }));
    }
    pub fn parse_reference_type_mut(&mut self) -> Ast<TypeNode> {
        self.parse(TokenKind::Keyword(Keyword::Mut));

        return Ast::Parsed(TypeNode::ReferenceType(ReferenceType {
            kind: RefKind::Mut,
            argument: Box::new(self.parse_type_node()),
        }));
    }
}
