use crate::{
    ast::{ast::Ast, TypeNode},
    parser::Parser,
};

impl Parser {
    pub fn parse_tuple_type(&mut self) -> Ast<TypeNode> {
        // TODO: parse types
        Ast::Error("".to_string())
    }
}
