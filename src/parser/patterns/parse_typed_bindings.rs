use crate::{
    ast::{ast::Ast, TypedBinding},
    lexer::TokenKind,
    parser::Parser,
};

impl Parser {
    pub fn parse_typed_bindings(&mut self) -> Vec<Ast<TypedBinding>> {
        let mut typed_bindings: Vec<Ast<TypedBinding>> = vec![];
        println!("loop started");
        loop {
            typed_bindings.push(self.parse_typed_binding());
            if self.token() == TokenKind::Comma {
                self.next();
            } else {
                break;
            }
        }
        println!("loop started");
        typed_bindings
    }
}
