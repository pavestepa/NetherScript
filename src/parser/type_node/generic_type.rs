use crate::{
    ast::{ast::Ast, GenericType, Ident, TypeNode},
    lexer::TokenKind,
    parser::Parser,
};

impl Parser {
    pub fn parse_generic_type(&mut self) -> Ast<TypeNode> {
        if let TokenKind::Ident(ident) = self.current().kind {
            let ident = Ident(ident);
            self.parse(self.current().kind);

            let arguments = self.parse_generic_arguments();

            return Ast::Parsed(TypeNode::GenericType(GenericType { ident, arguments }));
        }
        Ast::Error("".to_string())
    }

    fn parse_generic_arguments(&mut self) -> Option<Vec<Ast<TypeNode>>> {
        match self.current().kind {
            TokenKind::Ident(_) => Some(vec![self.parse_type_node()]),
            TokenKind::Less => {
                self.parse(self.current().kind); // "<"
                let mut args = vec![];
                while self.current().kind != TokenKind::Greater {
                    args.push(self.parse_type_node());
                    if self.current().kind == TokenKind::Greater {
                        break;
                    }
                    self.parse(TokenKind::Comma);
                }
                let res = Some(args);
                self.parse(TokenKind::Greater);
                res
            }
            _ => None,
        }
    }
}
