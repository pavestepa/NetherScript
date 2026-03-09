use crate::{
    ast::StructDecl,
    parser::{parse::parse, Parse, Parser},
};

impl Parser {
    pub fn parse_struct_decl(&mut self) -> StructDecl {
        parse(StructDecl { fields: vec![] }, errors)
    }
}
