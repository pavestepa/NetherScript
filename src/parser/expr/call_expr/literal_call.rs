use crate::{ast::Call, parser::Parser};

impl Parser {
    pub fn parse_literal_call_boolean(&mut self) -> Call {}
    pub fn parse_literal_call_number(&mut self) -> Call {}
    pub fn parse_literal_call_string(&mut self) -> Call {}
}
