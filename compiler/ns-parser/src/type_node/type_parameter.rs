use ns_ast::{TypeParameter, ast::Ast};

use crate::Parser;



impl Parser {
    pub fn parse_type_parameter(&mut self) -> Ast<TypeParameter> {
        // TODO: parse types
        Ast::Error("".to_string())
    }
}
