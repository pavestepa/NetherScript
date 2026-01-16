use crate::{
    ast::TypeDecl,
    parser::{parse::parse, Parse, Parser},
};

impl Parser {
    pub fn parse_type_decl(&mut self) -> Parse<TypeDecl> {
        parse(TypeDecl {}, errors)
    }
}
