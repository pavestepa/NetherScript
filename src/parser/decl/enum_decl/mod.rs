use crate::{
    ast::EnumDecl,
    parser::{parse::parse, Parse, Parser},
};

impl Parser {
    pub fn parse_enum_decl(&mut self) -> Parse<EnumDecl> {
        parse(EnumDecl {}, errors)
    }
}
