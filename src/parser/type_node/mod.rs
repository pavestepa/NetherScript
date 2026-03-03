mod function_type;
mod generic_type;
mod tuple_type;
mod type_parameter;

use crate::{
    ast::{ast::Ast, TypeNode},
    parser::Parser,
};

impl Parser {
    pub fn parse_type_node(&mut self) -> Ast<TypeNode> {
        println!("[STARTED] parse TypeNode");
        // TODO: parse type node
        Ast::Error("".to_string())
    }
}
