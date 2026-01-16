use crate::{
    ast::ConstDecl,
    parser::{parse::parse, Parse, Parser},
};

impl Parser {
    pub fn parse_const_decl(&mut self) -> Parse<ConstDecl> {
        parse(ConstDecl::new(ident, data_type, val), errors)
    }
}
