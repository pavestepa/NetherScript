use crate::{
    ast::{ast::Ast, Binding},
    lexer::TokenKind,
    parser::Parser,
};

impl Parser {
    pub fn parse_typed_bindings(&mut self) -> Vec<Ast<Binding>> {
        let mut typed_bindings: Vec<Ast<Binding>> = vec![];
        println!("loop started");
        loop {
            typed_bindings.push(self.parse_typed_binding());
            if self.current().kind == TokenKind::Comma {
                self.consume(TokenKind::Comma);
            } else {
                break;
            }
        }
        println!("loop started");
        typed_bindings
    }
}
