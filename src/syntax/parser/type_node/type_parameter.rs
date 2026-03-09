use crate::syntax::{
    ast::{ast::Ast, TypeNode, TypeParameter},
    parser::Parser,
};

impl Parser {
    pub fn parse_type_parameter(&mut self) -> Ast<TypeParameter> {
        // TODO: parse types
        Ast::Error("".to_string())
    }
}
