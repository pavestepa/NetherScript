use crate::{
    ast::TypeDecl,
    parser::{parse::parse, Parse, Parser},
};

impl Parser {
    pub fn parse_type_decl(&mut self) -> TypeDecl {
        parse(TypeDecl {}, errors)
    }
}
