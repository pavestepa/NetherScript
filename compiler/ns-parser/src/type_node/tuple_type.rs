use ns_ast::{TypeNode, ast::Ast};

use crate::Parser;

impl Parser {
    pub fn parse_tuple_type(&mut self) -> Ast<TypeNode> {
        // TODO: parse types
        Ast::Error("".to_string())
    }
}
